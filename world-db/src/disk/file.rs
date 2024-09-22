use std::io::SeekFrom;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

use prelude::*;

pub struct WriteFile{
	file: _File,
	pos: u64,
}

impl WriteFile {
	/// Does create if it does not exist
	pub async fn init<T: Deref<Target=PathBuf>>(path: T) -> Result<Self>{
		Ok(Self{file:_File::init(path, true).await?, pos: 0 })
	}
	/// Doesn't have bounds check
	pub async fn write(&mut self, pos: u64, bytes: &[u8]) -> Result<()>{
		self.file.seek(pos).await?;
		self.file.file.write_all(bytes).await?;
		self.file.file.flush().await?;
		self.file.seek(self.pos).await?;
		Ok(())
	}
	/// Bytes starting at `pos` will be move `space` forward.
	pub async fn insert_space(&mut self, pos: u64, space: u64) -> Result<()>{
		self.file.seek(pos).await?;
		let file = &mut self.file.file;
		let mut buf = Vec::with_capacity(100);
		file.read_to_end(&mut buf).await?;
		file.set_len(pos + buf.len() as u64 + space).await?;
		file.seek(SeekFrom::Start(pos + space)).await?;
		file.write_all(&buf).await?;
		self.file.seek(self.pos).await?;
		Ok(())
	}
	pub async fn read(self) -> ReadFile{
		ReadFile{
			file: self.file
		}
	}
}
pub struct ReadFile{
	file: _File
}

impl ReadFile {
	pub async fn init<T: Deref<Target=PathBuf>>(path: T) -> Result<Self>{
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

	pub async fn init<T: Deref<Target=PathBuf>>(path: T, write: bool) -> Result<Self>{

		let file = if write {
			OpenOptions::new().read(true).write(true).create(true).truncate(false).open(&*path).await?
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
			// self.file.seek(SeekFrom::Current(pos as i64 - self.position as i64)).await?; //FIXME: I want to use current, for (performance?), but this has issues...
			self.file.seek(SeekFrom::Start(pos)).await?;
			self.position = pos;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn write_read() -> Result<()> {
		let data = "test stuff".as_bytes().to_vec();
		let file_wrapper = tempfile::NamedTempFile::new()?;
		let mut wrapper = WriteFile::init(&file_wrapper.path().to_path_buf()).await?;
		wrapper.write(0,&data).await?;
		let mut read_wrapper = vec![0;data.len()];
		wrapper.read().await.read(0, &mut read_wrapper).await?;

		assert_eq!(read_wrapper, data);
		
		drop(file_wrapper);
		
		Ok(())
	}

	#[tokio::test]
	async fn write_read_multiple() -> Result<()> {
		let data_1 = "bark bark bark".as_bytes().to_vec();
		let data_2 = "quack quack".as_bytes().to_vec();
		
		let start = data_1.len() as u64 + 10;

		let file_wrapper = tempfile::NamedTempFile::new()?;
		let mut wrapper = WriteFile::init(&PathBuf::from("/tmp/test_file")).await?;
		wrapper.write(0,&data_1).await?;
		wrapper.write(start, &data_2).await?;
		
		let mut wrapper = wrapper.read().await;
		
		let mut read = vec![0; data_1.len()];
		wrapper.read(0, &mut read).await?;
		assert_eq!(read, data_1);

		let mut read = vec![0; data_2.len()];
		wrapper.read(start, &mut read).await?;
		assert_eq!(read, data_2);

		drop(file_wrapper);

		Ok(())
	}

	#[tokio::test]
	async fn insert_space() -> Result<()>{
		let data_1 = "semente by snarky puppy".as_bytes().to_vec();
		let data_2 = "5 little ducks".as_bytes().to_vec();
		
		let file_wrapper = tempfile::NamedTempFile::new()?;
		let mut wrapper = WriteFile::init(&file_wrapper.path().to_path_buf()).await?;
		wrapper.write(10,&data_1).await?;
		wrapper.write(10 + data_1.len() as u64, &data_2).await?; //issue here, being written to the wrong spot.
		wrapper.insert_space(10 + data_1.len() as u64, 100).await?;
		
		let mut wrapper = wrapper.read().await;
		
		let mut read = vec![0; data_1.len()];
		wrapper.read(10, &mut read).await?;
		assert_eq!(read, data_1);

		let mut read = vec![0; data_2.len()];
		wrapper.read(110 + data_1.len() as u64, &mut read).await?;
		assert_eq!(read, data_2);
		Ok(())
	}
}