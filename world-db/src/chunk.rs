use core::hash;
use std::hash::BuildHasherDefault;

use itertools::Itertools;
use rustc_hash::{FxHasher, FxHashMap};
use serde::{Deserialize, Serialize};
use sync::RwLock;
use tokio::sync;

use core_obj::Registrar;
use world_protocol::chunks::ReadChunk as ReadChunkTrait;
use world_protocol::chunks::WriteChunk as WriteChunkTrait;
use world_protocol::pos::{ChunkLocalPos, ChunkPos};

use crate::arr3d::Arr3d;
use crate::disk::rle::Arr3dRLE;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChunkData<R: Registrar>{
    pub(crate) voxels: Arr3d<Option<R::VoxelType>>,
    pub(crate) voxel_data: FxHashMap<ChunkLocalPos, R::DataContainer>
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
    pub(crate) guard: sync::RwLockReadGuard<'a, ChunkData<R>>,
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
    pub(crate) guard: sync::RwLockWriteGuard<'a, ChunkData<R>>,
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


#[derive(Debug, Deserialize, Serialize)]
pub struct SmallerChunk<R: Registrar>{
    voxels: Arr3dRLE<Option<R::VoxelType>>,
    data: Vec<(ChunkLocalPos, R::DataContainer)>,
}

impl<R: Registrar> SmallerChunk<R> {
    pub fn new(chunk: &ChunkData<R>) -> Self{
        Self{
            voxels: Arr3dRLE::from(chunk.voxels.clone()),
            data: chunk.voxel_data.iter().map(|(&k,v)|(k,v.clone())).collect_vec(),
        }
    }
    pub fn to_data(self) -> ChunkData<R>{
        let hasher: BuildHasherDefault<FxHasher> = hash::BuildHasherDefault::default();
        let mut voxel_data = FxHashMap::with_capacity_and_hasher(self.data.len(), hasher);
        voxel_data.extend(self.data);
        ChunkData{
            voxels: self.voxels.into(),
            voxel_data,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use core_obj::fake::{FakeRegistrar, FakeVoxel};

    use crate::PosU;

    use super::*;

    #[tokio::test]
    async fn round_trip_bincode() {
        let chunk: Chunk<FakeRegistrar> = Chunk::new();
        let read = chunk.read(Default::default()).await;
        let data = read.guard.deref();
        let mut serialized = bincode::serialize(data).unwrap();
        let deserialized: ChunkData<FakeRegistrar> = bincode::deserialize(&serialized).unwrap();
        assert_eq!(*data, deserialized);
    }

    #[tokio::test]
    /// I was worried that this would cause an error? It does not.
    async fn serialize_blank() {
        let blank = vec![0u8;32779 + 1];
        let deserialized: ChunkData<FakeRegistrar> = bincode::deserialize(&blank).unwrap();
    }

    #[tokio::test]
    async fn round_trip_smaller_chunk(){
        let mut chunk_data: ChunkData<FakeRegistrar> = ChunkData::default();
        *chunk_data.voxels.get_mut(PosU(0,4,2)) = Some(FakeVoxel(33));
        let smaller = SmallerChunk::new(&chunk_data);
        let conv_chunk = smaller.to_data();
        assert_eq!(conv_chunk, chunk_data);

    }
}
