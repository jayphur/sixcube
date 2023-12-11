use core_obj::{Pos, Runtime};
use rustc_hash::FxHashMap;
use world_protocol::{message::VoxelMsg, VisitorRegistry};
use chunk::{Chunk, ChunkData};

mod chunk;

// Tree tree tree tree tree!!!!!
//Smallest chunk 32^3
//nah maybe 8^3

const CHUNK_SIZE: usize = 32;
const CHUNK_SIZE_I32: i32 = 32;

#[derive(Debug)]
pub struct Map<R> 
where 
R: Runtime, 
{
    active_chunks: FxHashMap<Pos16, ChunkData<R>>
}
impl<R> world_protocol::Map<R> for Map<R> 
where 
R: Runtime, 
{
    fn get_type(&self, pos: Pos) -> Option<R::VoxelType> {
        todo!()
    }

    fn msg_voxel(&self, pos: Pos, msg: VoxelMsg<R>) {
        todo!()
    }

    fn load(&mut self, pos: &[Pos]) {
        todo!()
    }

    fn update<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, R, Self> {
        todo!()
    }
}
impl<R> Default for Map<R> 
where 
R: Runtime, 
{
    fn default() -> Self {
        todo!()
    }
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct Pos16(pub i16,pub i16,pub i16); 
pub struct LocalPos(pub u8,pub u8,pub u8);