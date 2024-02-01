use sync::RwLock;
use tokio::sync;

use core_obj::Runtime;
use world_protocol::chunks::ReadChunk as ReadChunkTrait;
use world_protocol::chunks::WriteChunk as WriteChunkTrait;
use world_protocol::pos::ChunkLocalPos;

use crate::arr3d::Arr3d;

#[derive(Debug)]
pub struct Chunk<R: Runtime>{
    lock: RwLock<ChunkData<R>>
}

impl<R: Runtime> Chunk<R> {
    pub async fn read<'a>(&'a self) -> ReadChunk<'a, R>{
        ReadChunk{
            lock: self.lock.read().await,
        }
    }
    pub async fn write<'a>(&'a self) -> WriteChunk<'a, R>{
        WriteChunk{
            lock: self.lock.write().await,
        }
    }
}

#[derive(Debug, Clone)]
struct ChunkData<R: Runtime>{
    voxels: Arr3d<Option<R::VoxelType>>
}

impl<R: Runtime> ChunkData<R> {
}

pub struct ReadChunk<'a, R> where R: Runtime {
    lock: sync::RwLockReadGuard<'a, ChunkData<R>>,
}

impl<'a, R> ReadChunkTrait<R> for ReadChunk<'a, R>
where R: Runtime
{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<<R as Runtime>::VoxelType> {
        *self.lock.voxels.get(pos.into())
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
        *self.lock.voxels.get(pos.into())
    }

    fn get_data(&self, pos: ChunkLocalPos) -> Option<()> {
        todo!()
    }
}

impl<'a, R> WriteChunkTrait<R> for WriteChunk<'a, R>
where R: Runtime
{
    fn set_type(&mut self, pos: ChunkLocalPos, r#type: <R as Runtime>::VoxelType) {
        self.lock.voxels.get_mut(pos.into()) = r#type;
    }

    fn set_data(&mut self, pos: ChunkLocalPos, data: ()) {
        todo!()
    }
}