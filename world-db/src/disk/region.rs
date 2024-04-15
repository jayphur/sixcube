//! 16\*16\*16 chunks
//TODO: spawn_blocking for the convertion stuff

use std::io::SeekFrom;
use std::marker::PhantomData;
use std::ops::DerefMut;

use bincode::Options;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeek, AsyncSeekExt, AsyncWriteExt};

use prelude::*;

use crate::disk::BINCODE_OPTIONS;
use crate::disk::lookup_table::{LOOKUP_TABLE_BYTE_SIZE, LookupTable};
use crate::PosU;

const PADDING_BYTES: u64 = {4096};


///Region don't use x y z just x
fn to_index<T:Into<usize>>(xyz: (T, T, T)) -> usize{
	xyz.0.into() + xyz.1.into() * 16 + xyz.2.into() * 16 * 16
}

///This should have exclusive ownership over the file
pub struct RegionFile<T: Serialize + for <'de> Deserialize<'de>  + Debug>{
	file: File,
	lookup_table: LookupTable,
	buffer: Vec<u8>,
	needs_to_be_closed: bool,
	__marker: PhantomData<T>
}

impl<T: Serialize + for<'de> Deserialize<'de> + Debug> Drop for RegionFile<T> {
	fn drop(&mut self) {
		if self.needs_to_be_closed {
			panic!("RegionFile instance performed writing did not run `self.close()` before being dropped. Please run!")
		}
	}
}

impl<T: Serialize + for <'de> Deserialize<'de> + Debug> RegionFile<T> {
	pub async fn init(mut file: File) -> Result<Self>{
		let mut buf = Box::new([0u8;LOOKUP_TABLE_BYTE_SIZE]);
		file.seek(SeekFrom::Start(0)).await?;
		let lookup_table = match file.read_exact(buf.as_mut()).await{
			Ok(LOOKUP_TABLE_BYTE_SIZE) =>{
				LookupTable::init_from_bytes(buf.as_slice())
			}
			Ok(_) => {
				file.write(&LookupTable::default().to_bytes()).await
					.with_context(||format!("attempted to write new lookup table to file {:?} but encountered error",file))?;
				Ok(LookupTable::default())
			},
			Err(err) => {
				Ok(if let std::io::ErrorKind::UnexpectedEof = err.kind(){
					file.write(&LookupTable::default().to_bytes()).await
						.with_context(||format!("Lookup table did not exist in {:?}, error encountered when writing default lookup table to file.",file))?;
					LookupTable::default()
				} else {
					return Err(anyhow!("Issue getting lookup table in region file. Error: {}", err));
				})
			}
		};

		Ok(Self{
			file,
			lookup_table: lookup_table?,
			buffer: vec![],
			needs_to_be_closed: false,
			__marker: Default::default(),
		})
	}

	pub async fn read(&mut self, pos: PosU) -> Result<Option<T>>{
		let index = to_index(pos.tuple());
		let start = self.lookup_table.start(index);
		let length = self.lookup_table.length_of(index);
		if length == 0 {
			return Ok(None);
		}
		let mut buffer = vec![0u8;length as usize];
		let seek_start = start + LOOKUP_TABLE_BYTE_SIZE as u64;
		self.file.seek(SeekFrom::Start(seek_start)).await
			.with_context(||format!("failed to seek file at {seek_start}"))?;

		self.file.read_exact(&mut buffer).await
			.with_context(||format!("failed reading the length of {} to buffer ", buffer.len()))?;
		let deserialized =  BINCODE_OPTIONS.deserialize::<T>(buffer.as_slice())
			.with_context(||format!("failed deserializing from buffer of length {}, bytes are probably corrupted or misaligned.", buffer.len()))?;
		Ok(Some(deserialized))
	}
	//FIXME: issue with writing large arrays where we hit a stack overflow, probably due to the serde encoding for the 3d array using recursion.
	pub async fn write(&mut self, pos: PosU, t: &T) -> Result<()>{
		self.needs_to_be_closed = true;
		if pos.0 > 15 || pos.1 > 15 || pos.2 > 15{
			return Err(anyhow!("attempting to write to region file at {:?}, out of bounds. Positions must be \"[0,16)\"",pos)	);
		}
		let index = to_index(pos.tuple());
		let encoded=  BINCODE_OPTIONS.serialize(&t)?;
		let mut full_length = (encoded.len() as u64).div_ceil(PADDING_BYTES) * PADDING_BYTES; // in bytes
		let shifted = self.lookup_table.fit(index, full_length);
		self.lookup_table.set_padding(index, full_length -encoded.len() as u64);
		let start = self.lookup_table.start(index) + LOOKUP_TABLE_BYTE_SIZE as u64;
		self.shift(start, shifted as i64).await?;
		self.file.seek(SeekFrom::Start(start)).await?;
		self.file.write_all(&encoded).await?;
		self.file.flush().await?;
		Ok(())
	}

	///shift everything back in the file, doesn't touch the lookup table.
	pub async fn shift(&mut self,pos: u64, amount: i64) -> Result<()>{
		if amount == 0 {return Ok(())}
		let amount = amount.div_euclid(PADDING_BYTES as i64)*PADDING_BYTES as i64;
		self.file.seek(SeekFrom::Start(pos)).await?;
		self.buffer.clear();
		self.file.read_to_end(&mut self.buffer).await?;
		let new_length = LOOKUP_TABLE_BYTE_SIZE as i64 + self.lookup_table.end as i64 + amount;
		self.file.set_len(new_length as u64).await?;
		self.file.seek(SeekFrom::Start( (pos as i64 + amount) as u64)).await?;
		self.file.write_all(&self.buffer).await?;

		Ok(())
	}

	/// Run once finished with the region file instance
	pub async fn close(mut self) -> Result<()>{
		if self.needs_to_be_closed {
			self.file.seek(SeekFrom::Start(0)).await?;
			let table_bytes = self.lookup_table.to_bytes();
			self.file.write_all(&table_bytes).await?;
			self.file.flush().await?;
			self.needs_to_be_closed = false;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::io::SeekFrom;

	use bincode::Options;
	use tokio::io::{AsyncReadExt, AsyncSeekExt};

	use prelude::Result;

	use crate::chunk::{ChunkData, SmallerChunk};
	use crate::disk::BINCODE_OPTIONS;
	use crate::disk::lookup_table::LOOKUP_TABLE_BYTE_SIZE;
	use crate::disk::region::{RegionFile, to_index};
	use crate::PosU;

	#[tokio::test]
	async fn writing_correct_bytes() -> Result<()>{
		let chunk1 = ChunkData::test_chunk(329857);
		let chunk2 = ChunkData::test_chunk(25657);
		let chunk1_bytes = BINCODE_OPTIONS.serialize(&SmallerChunk::new(&chunk1)).unwrap();
		let chunk2_bytes =  BINCODE_OPTIONS.serialize(&SmallerChunk::new(&chunk2)).unwrap();
		let temp_file = tempfile::NamedTempFile::new()?;
		let file = tokio::fs::File::options().write(true).read(true).open(temp_file.path()).await?;
		let mut region: RegionFile<SmallerChunk> = RegionFile::init(file).await?;
		let mut buf = vec![];

		region.write(PosU(1,0,0), &SmallerChunk::new(&chunk2)).await?;
		region.write(PosU(0,0,0), &SmallerChunk::new(&chunk1)).await?;

		let lookup_table = region.lookup_table.clone();
		region.close().await?;

		//Opening the file again for good measure.
		let mut file = tokio::fs::File::options().read(true).open(temp_file.path()).await?;

		//check if it contains chunk1_bytes at the advertised location
		let index = to_index(PosU(0, 0, 0).tuple());
		let start = lookup_table.start(index) + LOOKUP_TABLE_BYTE_SIZE as u64;
		let length = lookup_table.length_of(index);
		assert_eq!(length, chunk1_bytes.len() as u64);
		buf.resize(length as usize, 0);
		file.seek(SeekFrom::Start(start)).await?;
		file.read_exact(&mut buf).await?;
		assert_eq!(buf, chunk1_bytes);

		//check if it contains chunk2_bytes at the advertised location
		let index = to_index(PosU(1, 0, 0).tuple());
		let start = lookup_table.start(index) + LOOKUP_TABLE_BYTE_SIZE as u64;
		let length = lookup_table.length_of(index);
		assert_eq!(length, chunk2_bytes.len() as u64);
		buf.clear();
		buf.resize(length as usize, 0);
		file.seek(SeekFrom::Start(start)).await?;
		file.read_exact(&mut buf).await?;
		assert_eq!(buf, chunk2_bytes);

		drop(file);
		drop(temp_file);
		Ok(())
	}
	#[tokio::test]
	async fn mock_storage_round_trip() -> Result<()>{
		let data_1 = ChunkData::test_chunk(1);
		let data_2 = ChunkData::test_chunk(2);
		let data_3 = ChunkData::test_chunk(3);

		let temp_file = tempfile::NamedTempFile::new().unwrap();
		let file = tokio::fs::File::options().write(true).read(true).open(temp_file.path()).await.unwrap();
		let mut region: RegionFile<SmallerChunk> = RegionFile::init(file).await.unwrap();

		region.write(PosU(1,0,0),&SmallerChunk::new(&data_2)).await.unwrap();
		region.write(PosU(0,0,0),&SmallerChunk::new(&data_1)).await.unwrap();
		region.write(PosU(0,2,0),&SmallerChunk::new(&data_3)).await.unwrap();

		region.close().await?;

		let file = tokio::fs::File::options().read(true).open(temp_file.path()).await.unwrap();
		let mut region: RegionFile<SmallerChunk> = RegionFile::init(file).await.unwrap();

		let read = region.read(PosU(0,0,0)).await?.unwrap();
		assert_eq!(data_1, read.to_chunk()?);

		let read = region.read(PosU(1,0,0)).await?.unwrap();
		assert_eq!(data_2, read.to_chunk()?);

		let read = region.read(PosU(0,2,0)).await?.unwrap();
		assert_eq!(data_3, read.to_chunk()?);

		region.close().await?;

		drop(temp_file);
		Ok(())
	}

}