use async_trait::async_trait;
use chashmap_async::CHashMap;

pub use chunk::{ReadChunk, WriteChunk};
use prelude::*;

use crate::{ChunkPos, Error};
use crate::map::chunk::ChunkData;

pub(crate) mod chunk;
pub(crate) mod arr3d;
#[derive(Debug)]
pub struct Map{
	chunks: CHashMap<ChunkPos, ChunkData>,
}

#[async_trait]
impl crate::MapTrait for Map {
	type ReadChunk<'a> = ReadChunk<'a>;
	type WriteChunk<'a> = WriteChunk<'a>;

	async fn read<'a>(&'a self, pos: ChunkPos) -> Result<Option<Self::ReadChunk<'a>>, Error> {
		if let Some(guard) =  self.chunks.get(&pos).await{
			Ok(Some(ReadChunk{
				guard,
			}))
		} else {
			Ok(None)
		}
	}

	async fn write<'a>(&'a self, pos: ChunkPos) -> Result<Self::WriteChunk<'a>, Error> {
		if let Some(guard) = self.chunks.get_mut(&pos).await{
			Ok(WriteChunk{
				guard,
			})
		} else {
			self.chunks.insert(pos, ChunkData::default()).await.unwrap();
			Ok(WriteChunk{
				guard: self.chunks.get_mut(&pos).await.unwrap(),
			})
		}
	}

	async fn clone(&self) -> Self {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	#[test]
	pub fn it_works() {
	    assert_eq!(2, 1+1);
	}
}