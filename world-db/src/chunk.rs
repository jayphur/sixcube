use rustc_hash::FxHashMap;
use sync::RwLock;
use tokio::sync;

use core_obj::Registrar;
use world_protocol::chunks::ReadChunk as ReadChunkTrait;
use world_protocol::chunks::WriteChunk as WriteChunkTrait;
use world_protocol::pos::{ChunkLocalPos, ChunkPos};

use crate::arr3d::Arr3d;

//TODO: special solid and empty variants (NOTE: enum's are as large as their largest variant, don't use just an enum)
#[derive(Debug)]
pub struct Chunk<R: Registrar>{
    lock: RwLock<ChunkData<R>>,
}

impl<R: Registrar> Chunk<R> {
    pub fn new() -> Self{
        Self{
            lock: RwLock::new(ChunkData::default())
        }
    }
    pub async fn read<'a>(&'a self, c_pos: ChunkPos) -> ReadChunk<'a, R>{
        ReadChunk{
            guard: self.lock.read().await,
        }
    }
    pub async fn write<'a>(&'a self, c_pos: ChunkPos) -> WriteChunk<'a, R>{
        WriteChunk{
            guard: self.lock.write().await,
        }
    }
}

#[derive(Debug, Clone)]
struct ChunkData<R: Registrar>{
    voxels: Arr3d<Option<R::VoxelType>>,
    voxel_data: FxHashMap<ChunkLocalPos, R::DataContainer>
}

impl<R: Registrar> Default for ChunkData<R> {
    fn default() -> Self {
        Self{
            voxels: Default::default(),
            voxel_data: Default::default(),
        }
    }
}

impl<R: Registrar> ChunkData<R> {
}

pub struct ReadChunk<'a, R> where R: Registrar {
    guard: sync::RwLockReadGuard<'a, ChunkData<R>>,
}

impl<'a, R> ReadChunkTrait<R> for ReadChunk<'a, R>
where R: Registrar
{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<<R as Registrar>::VoxelType> {
        *self.guard.voxels.get(pos.into())
    }

    fn get_data(&self, pos: ChunkLocalPos) -> Option<&R::DataContainer> {
        self.guard.voxel_data.get(&pos)
    }
}




pub struct WriteChunk<'a, R> where R: Registrar {
    guard: sync::RwLockWriteGuard<'a, ChunkData<R>>,
}

impl<'a, R> ReadChunkTrait<R> for WriteChunk<'a, R>
where R: Registrar
{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<R::VoxelType> {
        *self.guard.voxels.get(pos.into())
    }

    fn get_data(&self, pos: ChunkLocalPos) -> Option<&R::DataContainer> {
        self.guard.voxel_data.get(&pos)
    }
}

impl<'a, R> WriteChunkTrait<R> for WriteChunk<'a, R>
where R: Registrar
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