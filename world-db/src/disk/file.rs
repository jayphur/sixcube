use std::io::SeekFrom;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

use prelude::*;

pub struct WriteFile{
	file: _File
}

impl WriteFile {
	/// Does create if it does not exist
	pub async fn init(path: Arc<PathBuf>) -> Result<Self>{
		Ok(Self{file:_File::init(path, true).await?})
	}
	pub async fn write(&mut self, pos: u64, bytes: &[u8]) -> Result<()>{
		self.file.seek(pos).await?;
		self.file.file.write_all(bytes).await?;
		Ok(())
	}
	pub async fn insert_space(&mut self, pos: u64, space: u64) -> Result<()>{
		self.file.seek(pos).await?;
		let mut buf = Vec::with_capacity(100);
		self.file.file.read_to_end(&mut buf).await?;
		self.file.file.set_len(pos + buf.len() as u64 + space).await?;
		self.file.file.seek(SeekFrom::Current(space as i64)).await?;
		self.file.file.write_all(&buf).await?;
		Ok(())
	}
	pub async fn read(self) -> ReadFile{
		ReadFile{
			file: self.file
		}
	}
	pub async fn finished(&mut self) -> Result<()>{
		self.file.file.flush().await?;
		Ok(())
	}
}
pub struct ReadFile{
	file: _File
}

impl ReadFile {
	pub async fn init(path: Arc<PathBuf>) -> Result<Self>{
		Ok(Self{file:_File::init(path, false).await?})
	}

	pub async fn read(&mut self, pos: u64, bytes: &mut [u8]) -> Result<()>{
		self.file.seek(pos).await?;
		self.file.file.read_exact(bytes).await?;
		Ok(())
	}
}

struct _File{
	file: File,
	position: u64,
}

impl _File {

	pub async fn init(path: Arc<PathBuf>, write: bool) -> Result<Self>{

		let file = if write {
			OpenOptions::new().read(true).write(true).create(true).open(&*path).await?
		} else {
			OpenOptions::new().read(true).write(false).open(&*path).await?
		};
		Ok(Self{
			file,
			position: 0,
		})
	}
	pub async fn seek(&mut self, pos: u64) -> Result<()>{
		if pos == 0{
			self.file.seek(SeekFrom::Start(0)).await?;
			self.position = 0;
		} else {
			self.file.seek(SeekFrom::Current(pos as i64 - self.position as i64)).await?;
			self.position = pos;
		}
		Ok(())
	}
}