use std::io::{ErrorKind, SeekFrom};
use std::path::PathBuf;
use std::sync::Arc;

use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

use core_obj::PosU;
use prelude::*;

use crate::disk::{EncodedChunk, RegionFileInfo};
use crate::disk::file::WriteFile;
use crate::disk::lookup_table::{LOOKUP_TABLE_BYTE_LENGTH, LookupTable};

fn to_index<T:Into<usize>>(xyz: (T, T, T)) -> usize{
	xyz.0.into() + xyz.1.into() * 16 + xyz.2.into() * 16 * 16
}

pub async fn create_region_file(info: &RegionFileInfo) -> Result<()>{
	let mut file = OpenOptions::new().write(true).create(true).open(&*info.path).await?;
	file.set_len(info.table.end()).await?;
	file.seek(SeekFrom::Start(0)).await?;
	let table_bytes = info.table.to_bytes();
	file.write(&table_bytes).await?;
	Ok(())
}
/// Reading has the possibility of corruption.
pub async fn get_table(path: &Arc<PathBuf>) -> Result<LookupTable,RegionError>{
	let result = get_table__(path).await;
	match result{
		Ok(buf) => {
			if let Ok(table) = LookupTable::from_bytes(&buf){
				Ok(table)
			} else {
				Err(RegionError::ContentsCorrupted)
			}
		},
		Err(err) => Err(RegionError::from_tokio_io_error(err))
	}
}
async fn get_table__(path: &Arc<PathBuf>) -> Result<Vec<u8>,tokio::io::Error>{
	let mut file = OpenOptions::new().write(false).read(true).open(path.as_ref()).await?;
	file.seek(SeekFrom::Start(0)).await?;
	let mut buf = Vec::with_capacity(LOOKUP_TABLE_BYTE_LENGTH);
	file.read_exact(&mut buf).await?;
	return Ok(buf);
}
/// Reading has the possibility of corruption.
pub async fn read(info: &RegionFileInfo, pos: PosU) -> Result<Option<EncodedChunk>,RegionError>{
	//translating IO errors into `RegionError`s
	let result = read__(info, pos).await;
	match result{
		Ok(option) => {
			let Some(vec) = option else {
				return Ok(None)
			};
			Ok(Some(EncodedChunk(vec)))
		},
		Err(err) => Err(RegionError::from_tokio_io_error(err))
	}
}
async fn read__(info: &RegionFileInfo, pos: PosU) -> Result<Option<Vec<u8>>, tokio::io::Error>{
	let mut file = OpenOptions::new().write(false).read(true).open(&*info.path).await?;
	let index = to_index(pos.tuple());
	let start=  info.table.start(index);
	let length = info.table.length_of(index);
	if length == 0{
		return Ok(None);
	}
	file.seek(SeekFrom::Start(start)).await?;
	let mut buf = Vec::with_capacity(length as usize);
	file.read_exact(&mut buf).await?;
	Ok(Some(buf))
}

pub async fn write(info: &mut RegionFileInfo, pos: PosU, data: &EncodedChunk) -> Result<(),RegionError>{
	let data = &data.0;
	let result  = write__(info, pos, &data).await;
	match result{
		Ok(_) => Ok(()),
		Err(err) => {
			//I don't THINK there is any acceptable error here.
			Err(RegionError::Other(err.into()))
		},
	}

}
async fn write__(info: &mut RegionFileInfo, pos: PosU, data: &[u8]) -> Result<()>{
	let table = &mut info.table;
	let mut file = WriteFile::init(info.path.clone()).await?;
	let index = to_index(pos.tuple());
	let shifted = table.fit(index, data.len() as u64);
	let start = table.start(index);
	if shifted != 0 {
		file.insert_space(start,shifted).await?
	}
	file.write(start, data).await?;
	file.finished().await?;
	Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum RegionError{
	#[error("Contents of region file corrupted")]
	ContentsCorrupted,
	#[error("Unable to find region file")]
	FileMissing,
	#[error("Tokio returned error: {0}")]
	Other(ErrorStruct),
}

impl RegionError {
	fn from_tokio_io_error(err: tokio::io::Error) -> Self{
		match err.kind(){
			ErrorKind::NotFound => RegionError::FileMissing,
			ErrorKind::UnexpectedEof => RegionError::ContentsCorrupted,
			_ => RegionError::Other(err.into())
		}
	}
}
