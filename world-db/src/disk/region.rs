//! 16\*16\*16 chunks


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
	buffer: Vec<u8>,
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
				file.write(&LookupTable::default().encode()).await?;
				LookupTable::default()
			},
			Err(err) => {
				if let std::io::ErrorKind::UnexpectedEof = err.kind(){
					file.write(&LookupTable::default().encode()).await?;
					LookupTable::default()
				} else {
					return Err(anyhow!("Issue getting lookup table in region file. Error: {}", err));
				}
			}
		};

		Ok(Self{
			file,
			buffer: vec![],
			lookup_table,
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
		self.buffer.resize(length as usize,0);
		self.file.seek(SeekFrom::Start(start + LOOKUP_TABLE_BYTE_SIZE as u64 )).await?;

		self.file.read_exact(&mut self.buffer).await?;
		let deserialized = bincode::deserialize::<SmallerChunk<R>>(self.buffer.as_slice())?;
		//FIXME: issue with deserializing. According to the buffer size, there is the right amount of bytes in it.
		Ok(Some(deserialized.to_data()))
	}
	//FIXME: issue with writing large arrays where we hit a stack overflow, probably due to the serde encoding for the 3d array using recursion.
	pub async fn write(&mut self, pos: PosU, data: &ChunkData<R>) -> Result<()>{
		let index = to_index(pos.tuple());
		self.buffer.clear();
		let encoded= bincode::serialize(&SmallerChunk::new(data))?;
		self.buffer.extend(		encoded);
		let mut length = (self.buffer.len() as u64) / PADDING_BYTES * PADDING_BYTES; // in bytes
		length += match self.buffer.len() as u64 % PADDING_BYTES {
			0 => 0,
			_ => PADDING_BYTES,
		};
		self.lookup_table.fit(index, length);
		self.lookup_table.set_padding(index,length-self.buffer.len() as u64);
		self.file.set_len(LOOKUP_TABLE_BYTE_SIZE as u64 + self.lookup_table.end ).await?;
		self.file.seek(SeekFrom::Start(self.lookup_table.start(index) + LOOKUP_TABLE_BYTE_SIZE as u64)).await?;
		self.file.write_all(&self.buffer).await?;
		self.file.flush().await?;
		Ok(())
	}
}

const LOOKUP_TABLE_BYTE_SIZE: usize = {(16*16*16*64 + 64 + 64) / 8};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// In bytes.
pub struct LookupTable{
	pub start: Vec<u64>,
	pub padding: Vec<u64>,
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
		self.length_of_with_padding(u) - self.padding[u]
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
			self.set_padding(index, self.padding[index] - extra);
		}

	}

	/// Declare that this section has `amount` bytes of padding in it.
	/// This does **NOT** expand anything.
	fn set_padding(&mut self, index:usize, amount: u64){
		self.padding[index] = amount
	}
	fn encode(&self) -> Vec<u8>{
		bincode::serialize(&self).unwrap()
	}
	fn decode(slice: &[u8]) -> Self{
		let decode: Self = bincode::deserialize(slice).unwrap();
		// if it's an empty boy, init.
		if decode.padding.len() == 0 || decode.start.len() == 0{
			return Self::default();
		}
		decode
	}
}
#[cfg(test)]
mod tests {
	use std::path::Path;

	use tokio::fs::OpenOptions;

	use core_obj::fake::{FakeRegistrar, FakeVoxel};
	use world_protocol::chunks::WriteChunk;
	use world_protocol::pos::ChunkLocalPos;

	use crate::chunk::Chunk;
	use crate::disk::region::{LOOKUP_TABLE_BYTE_SIZE, LookupTable, RegionFile};
	use crate::PosU;

	#[test]
	fn lookup_table_round_trip() {
		let mut table = LookupTable::default();
		*table.start.get_mut(2).unwrap() = 57;
		*table.start.get_mut(23).unwrap() = 48795;
		*table.start.get_mut(25).unwrap() = 48743295;
		assert_eq!( LookupTable::decode(&table.encode()), table); // Round Trip
	}
	#[test]
	fn lookup_table_bit_size(){
		let mut table = LookupTable::default();
		assert_eq!( table.start.len(), 16*16*16);
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
		let mut chunk: Chunk<FakeRegistrar> = Chunk::new();
		let mut write = chunk.write(Default::default()).await;
		*write.get_type_mut(ChunkLocalPos(0,0,0)) = Some(FakeVoxel(1));
		*write.get_type_mut(ChunkLocalPos(0,6,0)) = Some(FakeVoxel(6));
		*write.get_type_mut(ChunkLocalPos(12,9,3)) = Some(FakeVoxel(9));
		let data = write.guard.clone();
		//let file = File::from(tempfile::tempfile().unwrap());
		let file = OpenOptions::new().read(true).write(true).open(Path::new(&"/home/justin/testfile")).await.unwrap();
		file.set_len(0).await.unwrap();
		let mut region: RegionFile<FakeRegistrar> = RegionFile::init(file).await.unwrap();
		region.write(PosU(0,0,0),&data).await.unwrap();
		let read = region.read(PosU(0,0,0)).await.unwrap().unwrap();
		assert_eq!(data, read);
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