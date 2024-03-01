//! 16\*16\*16 chunks
//TODO: spawn_blocking for the convertion stuff

use std::io::SeekFrom;
use std::iter;
use std::marker::PhantomData;
use std::ops::DerefMut;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeek, AsyncSeekExt, AsyncWriteExt};

use core_obj::Registrar;
use prelude::*;

use crate::chunk::{ChunkData, SmallerChunk};
use crate::PosU;

const PADDING_BYTES: u64 = {4096};

///Region don't use x y z just x
fn to_index<T:Into<usize>>(xyz: (T, T, T)) -> usize{
	xyz.0.into() + xyz.1.into() * 16 + xyz.2.into() * 16 * 16
}

///This should have exclusive ownership over the file
pub struct RegionFile<R: Registrar>{
	file: File,
	lookup_table: LookupTable,
	__marker: PhantomData<R>
}

impl<R: Registrar> RegionFile<R> {
	pub async fn init(mut file: File) -> Result<Self>{
		let mut buf = Box::new([0u8;LOOKUP_TABLE_BYTE_SIZE]);
		let lookup_table = match file.read_exact(buf.as_mut()).await{
			Ok(LOOKUP_TABLE_BYTE_SIZE) =>{
				LookupTable::decode(buf.as_slice())
			}
			Ok(len) => {
				file.write(&LookupTable::default().encode()).await
					.with_context(||format!("attempted to write new lookup table to file {:?} but encountered error",file))?;
				Ok(LookupTable::default())
			},
			Err(err) => {
				Ok(if let std::io::ErrorKind::UnexpectedEof = err.kind(){
					file.write(&LookupTable::default().encode()).await
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
			__marker: Default::default(),
		})
	}

	pub async fn read(&mut self, pos: PosU) -> Result<Option<ChunkData<R>>>{
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
		let deserialized = bincode::deserialize::<SmallerChunk<R>>(buffer.as_slice())
			.with_context(||format!("failed deserializing from buffer of length {}, bytes are probably corrupted or misaligned.", buffer.len()))?;
		//FIXME: issue with deserializing. According to the buffer size, there is the right amount of bytes in it.
		Ok(Some(deserialized.to_data()))
	}
	//FIXME: issue with writing large arrays where we hit a stack overflow, probably due to the serde encoding for the 3d array using recursion.
	pub async fn write(&mut self, pos: PosU, data: &ChunkData<R>) -> Result<()>{
		if pos.0 > 15 || pos.1 > 15 || pos.2 > 15{
			return Err(anyhow!("attempting to write to region file at {:?}, out of bounds. Positions must be \"[0,16)\"",pos)	);
		}
		let index = to_index(pos.tuple());
		let encoded= bincode::serialize(&SmallerChunk::new(data))?;
		let mut length = (encoded.len() as u64) / PADDING_BYTES * PADDING_BYTES; // in bytes
		length += match encoded.len() as u64 % PADDING_BYTES {
			0 => 0,
			_ => PADDING_BYTES,
		};
		self.lookup_table.fit(index, length);
		self.lookup_table.set_padding(index,length-encoded.len() as u64);
		self.file.set_len(LOOKUP_TABLE_BYTE_SIZE as u64 + self.lookup_table.end ).await?;
		self.file.seek(SeekFrom::Start(self.lookup_table.start(index) + LOOKUP_TABLE_BYTE_SIZE as u64)).await?;
		self.file.write_all(&encoded).await?;
		self.file.flush().await?;
		Ok(())
	}
}

const LOOKUP_TABLE_BYTE_SIZE: usize = {((16*16*16)*64 + 64 + (16*16*16)*16) / 8 + 16}; //Plus 18 for some reason...
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// In bytes.
pub struct LookupTable{
	pub start: Vec<u64>,
	pub padding: Vec<u16>,
	pub end: u64,
}

impl Default for LookupTable {
	fn default() -> Self {
		Self{
			start: iter::repeat(0).take(16*16*16).collect_vec(),
			padding: iter::repeat(0).take(16*16*16).collect_vec(),
			end: 0,
		}
	}
}

impl LookupTable {
	fn start(&self, u: usize) -> u64{
		self.start[u]
	}
	fn length_of(&self, u: usize) -> u64{
		self.length_of_with_padding(u) - self.padding[u] as u64
	}
	fn length_of_with_padding(&self, u: usize) -> u64{
		if u >= self.start.len() - 1{
			self.end - self.start[u]
		} else {
			self.start[u+1] - self.start[u]
		}
	}

	///makes sure this index can fit this amount (or greater)
	fn fit(&mut self, index: usize, amount: u64){
		let current_length_full = self.length_of_with_padding(index);
		let current_length = self.length_of(index);

		if amount > current_length_full{
			self.padding[index] = 0;

			let amount = amount - current_length_full;
			//grow
			self.start.iter_mut()
				.skip(index + 1)
				.for_each(|val| *val += amount);
			self.end += amount;
		} else if amount > current_length{
			let extra = amount - current_length;
			self.set_padding(index, self.padding[index] as u64 - extra);
		}

	}

	/// Declare that this section has `amount` bytes of padding in it.
	/// This does **NOT** expand anything.
	fn set_padding(&mut self, index:usize, amount: u64){
		self.padding[index] = amount as u16
	}
	fn encode(&self) -> Vec<u8>{
		bincode::serialize(&self).unwrap()
	}
	fn decode(slice: &[u8]) -> Result<Self>{
		let decode: Self = bincode::deserialize(slice)?;
		// if it's an empty boy, init.
		if decode.padding.len() == 0 || decode.start.len() == 0{
			return Ok(Self::default());
		}
		Ok(decode)
	}
}
#[cfg(test)]
mod tests {
	use std::path::Path;

	use core_obj::fake::FakeRegistrar;

	use crate::chunk::ChunkData;
	use crate::disk::region::{LOOKUP_TABLE_BYTE_SIZE, LookupTable, RegionFile};
	use crate::PosU;

	#[test]
	fn lookup_table_round_trip() {
		let mut table = LookupTable::default();
		*table.start.get_mut(2).unwrap() = 57;
		*table.start.get_mut(23).unwrap() = 48795;
		*table.start.get_mut(25).unwrap() = 48743295;
		assert_eq!( LookupTable::decode(&table.encode()).unwrap(), table); // Round Trip
	}
	#[test]
	fn lookup_table_bit_size(){
		let mut table = LookupTable::default();
		assert_eq!( table.start.len(), 16*16*16);
		assert_eq!( table.padding.len(), 16*16*16);
		assert_eq!(table.encode().len(), LOOKUP_TABLE_BYTE_SIZE); // LOOKUP_TABLE_SIZE is accurate
	}

	#[test]
	fn lookup_table_padding() {
		let mut table = LookupTable::default();
		table.fit(0, 100);
		for x in 1..16*16*16 {
			assert_eq!(table.start(x), 100);
		}
		table.fit(0, 200);
		table.set_padding(0, 100);
		assert_eq!(table.length_of(0), 100);
		table.fit(0, 150);
		assert_eq!(table.length_of(0), 150);
		for x in 1..16*16*16 {
			assert_eq!(table.start(x), 200);
		}
		table.fit(0, 250);
		assert_eq!(table.length_of(0), 250);
		for x in 1..16*16*16 {
			assert_eq!(table.start(x), 250);
		}
	}

	#[tokio::test]
	async fn region_round_trip(){
		let data_1 = ChunkData::<FakeRegistrar>::test_chunk(1);
		let data_2 = ChunkData::<FakeRegistrar>::test_chunk(2);
		let data_3 = ChunkData::<FakeRegistrar>::test_chunk(3);

		let file = tokio::fs::File::create(Path::new("/home/justin/test".clone())).await.unwrap();
		//FIXME: "Bad file descriptor (os error 9)" WHY  WHY  WHY  WHY  WHY  WHY  WHY  WHY  WHY  WHY  WHY  WHY  WHY  WHY  WHY  WHY  WHY 
		let mut region: RegionFile<FakeRegistrar> = RegionFile::init(file).await.unwrap();

		region.write(PosU(0,0,0),&data_1).await.unwrap();
		region.write(PosU(1,0,0),&data_2).await.unwrap();
		region.write(PosU(0,2,0),&data_3).await.unwrap();

		let read = region.read(PosU(0,0,0)).await.unwrap().unwrap();
		assert_eq!(data_1, read);

		let read = region.read(PosU(1,0,0)).await.unwrap().unwrap();
		assert_eq!(data_2, read);

		let read = region.read(PosU(0,2,0)).await.unwrap().unwrap();
		assert_eq!(data_3, read);
	}

	#[test]
	fn lookup_table_fit() {
		let mut table = LookupTable::default();
		for x in (0..20) {
			table.fit(x, 10)
		}
		for x in (0..20) {
			table.fit(x, 9)
		}
		for x in (0..20) {
			assert_eq!(table.length_of(x), 10);
		}
		for x in (0..20) {
			assert_eq!(table.start(x), x as u64 *10);
		}
		table.fit(10, 25);
		for x in (0..10){
			assert_eq!(table.length_of(x), 10);
			assert_eq!(table.start(x), x as u64 *10);
		}
		assert_eq!(table.length_of(10),25);
		for x in (11..20){
			assert_eq!(table.length_of(x), 10);
			assert_eq!(table.start(x), x as u64 *10 + 15);
		}
		for x in (30..16*16*16-1){
			assert_eq!(table.length_of(x), 0);
		}
	}

	#[test]
	fn default_lookup_table() {
		let mut table = LookupTable::default();
		assert_eq!(table.padding.len(), 16*16*16);
		assert_eq!(table.start.len(), 16*16*16);
	}
}