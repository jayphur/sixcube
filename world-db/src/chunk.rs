use core::hash;
use std::hash::BuildHasherDefault;
use std::sync::Arc;

use itertools::Itertools;
use rustc_hash::{FxHasher, FxHashMap};
use serde::{Deserialize, Serialize};
use sync::RwLock;
use tokio::sync;

use core_obj::fake::{FakeRegistrar, FakeVoxel};
use core_obj::Registrar;
use prelude::*;
use world_protocol::chunks::ReadChunk as ReadChunkTrait;
use world_protocol::chunks::WriteChunk as WriteChunkTrait;
use world_protocol::pos::{ChunkLocalPos, ChunkPos};
use world_protocol::VoxEvent;

use crate::arr3d::Arr3d;
use crate::disk::rle::Arr3dRLE;
use crate::PosU;

//TODO: special solid and empty variants (NOTE: enum's are as large as their largest variant, don't use just an enum)
#[derive(Debug)]
pub struct Chunk<R: Registrar>{
    lock: RwLock<ChunkData<R>>, // Maybe Arc this?
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

impl ChunkData<FakeRegistrar> {
    ///Make some test chunk for testing.
    pub fn test_chunk(mut seed: usize) -> ChunkData<FakeRegistrar>{
        let numbers = [0,2,3,4,1,5,96,4,62,5,35,23,23,53,64,93,25,25,73,32,5,123,65,245,98,65,10,101];
        let mut data = ChunkData::default();
        for x in 0.. 128usize{
            let mut pos = PosU((x*seed*97)%16,(x*seed*11)%16,(x*seed*13)%16);
            if (x+seed)%3%2==0{
                for i in 0..x{
                    *data.voxels.get_mut(
                        PosU( (pos.0 + i ) % 16,pos.1,pos.2)
                    ) = Some(FakeVoxel(numbers[x*seed%numbers.len()]))
                }
            } else {
                *data.voxels.get_mut(pos) = Some(FakeVoxel(numbers[x*seed%numbers.len()]))
            }
        }
        data
    }

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
    fn peek_events(&self) -> &[Arc<VoxEvent<R>>] {
        todo!()
    }

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
    fn pop_event(&self) -> Option<Arc<VoxEvent<R>>> {
        todo!()
    }

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
    pub fn to_chunk(self) -> Result<ChunkData<R>>{
        let hasher: BuildHasherDefault<FxHasher> = hash::BuildHasherDefault::default();
        let mut voxel_data = FxHashMap::with_capacity_and_hasher(self.data.len(), hasher);
        voxel_data.extend(self.data);
        Ok(ChunkData{
            voxels: self.voxels.into_arr3d()?,
            voxel_data,
        })
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use core_obj::fake::FakeRegistrar;

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
        let chunk_data: ChunkData<FakeRegistrar> = ChunkData::test_chunk(2389);
        let smaller = SmallerChunk::new(&chunk_data);
        let conv_chunk = smaller.to_chunk().unwrap();
        assert_eq!(conv_chunk, chunk_data);
    }

    #[tokio::test]
    async fn round_trip_smaller_chunk_bin(){
        let chunk_data: ChunkData<FakeRegistrar> = ChunkData::test_chunk(985899);
        let bin_data = bincode::serialize(&chunk_data).unwrap();
        let smaller = SmallerChunk::new(&chunk_data);
        let bin_smaller = bincode::serialize(&smaller).unwrap();
        let conv_chunk = bincode::deserialize::<SmallerChunk<FakeRegistrar>>(&bin_smaller).unwrap().to_chunk().unwrap();
        assert_eq!(conv_chunk, chunk_data);

        println!("Size decrease: from {} to {}", bin_data.len(), bin_smaller.len());
    }
}

