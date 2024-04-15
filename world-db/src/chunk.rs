use core::hash;
use std::hash::BuildHasherDefault;

use itertools::Itertools;
use rustc_hash::{FxHasher, FxHashMap};
use serde::{Deserialize, Serialize};
use sync::RwLock;
use tokio::sync;

use core_obj::{DataContainer, VoxelId};
use core_obj::fake::fake_voxel;
use prelude::*;
use world_protocol::chunks::ReadChunk as ReadChunkTrait;
use world_protocol::chunks::WriteChunk as WriteChunkTrait;
use world_protocol::pos::{ChunkLocalPos, ChunkPos};

use crate::arr3d::Arr3d;
use crate::disk::rle::Arr3dRLE;
use crate::PosU;

//TODO: special solid and empty variants (NOTE: enum's are as large as their largest variant, don't use just an enum)
#[derive(Debug)]
pub struct Chunk{
    lock: RwLock<ChunkData>, // Maybe Arc this?
}

impl Chunk {
    pub fn new() -> Self{
        Self{
            lock: RwLock::new(ChunkData::default())
        }
    }
    pub async fn read<'a>(&'a self, c_pos: ChunkPos) -> ReadChunk<'a>{
        ReadChunk{
            guard: self.lock.read().await,
        }
    }
    pub async fn write<'a>(&'a self, c_pos: ChunkPos) -> WriteChunk<'a>{
        WriteChunk{
            guard: self.lock.write().await,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChunkData{
    pub(crate) voxels: Arr3d<Option<VoxelId>>,
    pub(crate) voxel_data: FxHashMap<ChunkLocalPos, DataContainer>
}

impl ChunkData {
    ///Make some test chunk for testing.
    pub fn test_chunk(mut seed: usize) -> ChunkData{
        let numbers = [0,2,3,4,1,5,96,4,62,5,35,23,23,53,64,93,25,25,73,32,5,123,65,245,98,65,10,101];
        let mut data = ChunkData::default();
        for x in 0.. 128usize{
            let mut pos = PosU((x*seed*97)%16,(x*seed*11)%16,(x*seed*13)%16);
            if (x+seed)%3%2==0{
                for i in 0..x{
                    *data.voxels.get_mut(
                        PosU( (pos.0 + i ) % 16,pos.1,pos.2)
                    ) = Some(fake_voxel(numbers[x*seed%numbers.len()]))
                }
            } else {
                *data.voxels.get_mut(pos) = Some(fake_voxel(numbers[x*seed%numbers.len()]))
            }
        }
        data
    }

}

impl Default for ChunkData {
    fn default() -> Self {
        Self{
            voxels: Default::default(),
            voxel_data: Default::default(),
        }
    }
}

impl ChunkData {
}

pub struct ReadChunk<'a> {
    pub(crate) guard: sync::RwLockReadGuard<'a, ChunkData>,
}

impl<'a> ReadChunkTrait for ReadChunk<'a>
{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<VoxelId> {
        *self.guard.voxels.get(pos.into())
    }

    fn get_data(&self, pos: ChunkLocalPos) -> Option<&DataContainer> {
        self.guard.voxel_data.get(&pos)
    }
}




pub struct WriteChunk<'a> {
    pub(crate) guard: sync::RwLockWriteGuard<'a, ChunkData>,
}

impl<'a, > ReadChunkTrait for WriteChunk<'a>{

    fn get_type(&self, pos: ChunkLocalPos) -> Option<VoxelId> {
        *self.guard.voxels.get(pos.into())
    }

    fn get_data(&self, pos: ChunkLocalPos) -> Option<&DataContainer> {
        self.guard.voxel_data.get(&pos)
    }
}

impl<'a> WriteChunkTrait for WriteChunk<'a>
{

    fn get_type_mut(&mut self, pos: ChunkLocalPos) -> &mut Option<VoxelId> {
        self.guard.voxels.get_mut(pos.into())
    }

    fn get_data_mut_weak(&mut self, pos: ChunkLocalPos) -> Option<&mut DataContainer> {
        Some(self.guard.voxel_data.get_mut(&pos)?)
    }


    fn get_data_mut(&mut self, pos: ChunkLocalPos) -> &mut DataContainer {
        if self.guard.voxel_data.contains_key(&pos) {
            self.guard.voxel_data.get_mut(&pos).unwrap()
        } else {
            self.guard.voxel_data.insert(pos, DataContainer::default());
            self.guard.voxel_data.get_mut(&pos).unwrap()
        }

    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct SmallerChunk{
    voxels: Arr3dRLE<Option<VoxelId>>,
    data: Vec<(ChunkLocalPos, DataContainer)>,
}

impl SmallerChunk {
    pub fn new(chunk: &ChunkData) -> Self{
        Self{
            voxels: Arr3dRLE::from(chunk.voxels.clone()),
            data: chunk.voxel_data.iter().map(|(&k,v)|(k,v.clone())).collect_vec(),
        }
    }
    pub fn to_chunk(self) -> Result<ChunkData>{
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

	use crate::chunk::{Chunk, ChunkData, SmallerChunk};

	#[tokio::test]
    async fn round_trip_bincode() {
        let chunk: Chunk = Chunk::new();
        let read = chunk.read(Default::default()).await;
        let data = read.guard.deref();
        let mut serialized = bincode::serialize(data).unwrap();
        let deserialized: ChunkData = bincode::deserialize(&serialized).unwrap();
        assert_eq!(*data, deserialized);
    }

    #[tokio::test]
    /// I was worried that this would cause an error? It does not.
    async fn serialize_blank() {
        let blank = vec![0u8;32779 + 1];
        let deserialized: ChunkData = bincode::deserialize(&blank).unwrap();
    }

    #[tokio::test]
    async fn round_trip_smaller_chunk(){
        let chunk_data: ChunkData = ChunkData::test_chunk(2389);
        let smaller = SmallerChunk::new(&chunk_data);
        let conv_chunk = smaller.to_chunk().unwrap();
        assert_eq!(conv_chunk, chunk_data);
    }

    #[tokio::test]
    async fn round_trip_smaller_chunk_bin(){
        let chunk_data: ChunkData = ChunkData::test_chunk(985899);
        let bin_data = bincode::serialize(&chunk_data).unwrap();
        let smaller = SmallerChunk::new(&chunk_data);
        let bin_smaller = bincode::serialize(&smaller).unwrap();
        let conv_chunk = bincode::deserialize::<SmallerChunk>(&bin_smaller).unwrap().to_chunk().unwrap();
        assert_eq!(conv_chunk, chunk_data);

        println!("Size decrease: from {} to {}", bin_data.len(), bin_smaller.len());
    }
}

