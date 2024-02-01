use rustc_hash::FxHashMap;
use sync::RwLock;
use tokio::sync;

use core_obj::Runtime;
use world_protocol::chunks::ReadChunk as ReadChunkTrait;
use world_protocol::chunks::WriteChunk as WriteChunkTrait;
use world_protocol::pos::ChunkLocalPos;

use crate::arr3d::Arr3d;

#[derive(Debug)]
pub struct Chunk<R: Runtime>{
    lock: RwLock<ChunkData<R>>,
}

impl<R: Runtime> Chunk<R> {
    pub async fn read<'a>(&'a self) -> ReadChunk<'a, R>{
        ReadChunk{
            guard: self.lock.read().await,
        }
    }
    pub async fn write<'a>(&'a self) -> WriteChunk<'a, R>{
        WriteChunk{
            guard: self.lock.write().await,
        }
    }
}

#[derive(Debug, Clone)]
struct ChunkData<R: Runtime>{
    voxels: Arr3d<Option<R::VoxelType>>,
    voxel_data: FxHashMap<ChunkLocalPos, R::DataContainer>
}

impl<R: Runtime> ChunkData<R> {
}

pub struct ReadChunk<'a, R> where R: Runtime {
    guard: sync::RwLockReadGuard<'a, ChunkData<R>>,
}

impl<'a, R> ReadChunkTrait<R> for ReadChunk<'a, R>
where R: Runtime
{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<<R as Runtime>::VoxelType> {
        *self.guard.voxels.get(pos.into())
    }

    fn get_data(&self, pos: ChunkLocalPos) -> Option<&R::DataContainer> {
        self.guard.voxel_data.get(&pos)
    }
}




pub struct WriteChunk<'a, R> where R: Runtime {
    guard: sync::RwLockWriteGuard<'a, ChunkData<R>>,
}

impl<'a, R> ReadChunkTrait<R> for WriteChunk<'a, R>
where R: Runtime
{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<R::VoxelType> {
        *self.guard.voxels.get(pos.into())
    }

    fn get_data(&self, pos: ChunkLocalPos) -> Option<&R::DataContainer> {
        self.guard.voxel_data.get(&pos)
    }
}

impl<'a, R> WriteChunkTrait<R> for WriteChunk<'a, R>
where R: Runtime
{
    fn get_type_mut(&mut self, pos: ChunkLocalPos) -> &mut Option<R::VoxelType> {
        self.guard.voxels.get_mut(pos.into())
    }

    fn get_data_mut_weak(&mut self, pos: ChunkLocalPos) -> Option<&mut R::DataContainer> {
        Some(self.guard.voxel_data.get_mut(&pos)?)
    }


    fn get_data_mut(&mut self, pos: ChunkLocalPos) -> &mut R::DataContainer {
        if self.guard.voxel_data.contains_key(&pos) {
            self.guard.voxel_data.get_mut(&pos).unwrap()
        } else {
            self.guard.voxel_data.insert(pos, R::DataContainer::default());
            self.guard.voxel_data.get_mut(&pos).unwrap()
        }

    }
}