use std::marker::PhantomData;

use core_obj::Runtime;
use tokio::sync;
use sync::RwLock;
use world_protocol::chunks::ReadChunk as ReadChunkTrait;
use world_protocol::chunks::WriteChunk as WriteChunkTrait;
use world_protocol::pos::ChunkLocalPos;

#[derive(Debug, Default)]
pub struct Chunk<R: Runtime>{
    lock: RwLock<ChunkData<R>>
}
#[derive(Debug, Default)]
struct ChunkData<R: Runtime>{
    __r: PhantomData<R>
}

pub struct ReadChunk<'a, R> where R: Runtime {
    lock: sync::RwLockReadGuard<'a, ChunkData<R>>,
}

impl<'a, R> ReadChunkTrait<R> for ReadChunk<'a, R>
where R: Runtime
{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<<R as Runtime>::VoxelType> {
        todo!()
    }

    fn get_data(&self, pos: ChunkLocalPos) -> Option<()> {
        todo!()
    }
}




pub struct WriteChunk<'a, R> where R: Runtime {
    lock: sync::RwLockWriteGuard<'a, ChunkData<R>>,
}

impl<'a, R> ReadChunkTrait<R> for WriteChunk<'a, R>
where R: Runtime
{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<<R as Runtime>::VoxelType> {
        todo!()
    }

    fn get_data(&self, pos: ChunkLocalPos) -> Option<()> {
        todo!()
    }
}

impl<'a, R> WriteChunkTrait<R> for WriteChunk<'a, R>
where R: Runtime
{
    fn set_type(&mut self, pos: ChunkLocalPos, r#type: <R as Runtime>::VoxelType) {
        todo!()
    }

    fn set_data(&mut self, pos: ChunkLocalPos, data: ()) {
        todo!()
    }
}