use std::marker::PhantomData;

use conversion::pos_to_local_pos;
use core_obj::{Pos, Voxel};
use world_protocol::{message::VoxelMsg, VisitorRead, VisitorRespond, VisitorApply, VisitorRegistry};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator};
use spatialtree::{OctTree, OctVec};
use chunk::Chunk;

use crate::conversion::pos_to_oct_pos;

mod chunk;


// Tree tree tree tree tree!!!!!

//Smallest chunk 32^3
//nah maybe 8^3

mod conversion;
type LocalPos = vector3d::Vector3d<usize>;

const CHUNK_SIZE: usize = 16;
const CHUNK_SIZE_I32: i32 = 16;

#[derive(Debug, Clone)]
pub struct Map<Vox> 
where 
Vox: core_obj::Voxel + Send + Sync, 
{
    loaded_chunks: OctTree<Chunk<Vox>, OctVec<u32>>,
    all_chunks: OctTree<(), OctVec<u32>>, //TODO: actually store "all chunks" not just ()
}
impl<Vox> world_protocol::Map<Vox> for Map<Vox> 
where 
Vox: core_obj::Voxel + Send + Sync, 
{
    fn get_type(&self, pos: Pos) -> Option<Vox::Type> {
        if let Some(chunk) = self.loaded_chunks.get_chunk_by_position(pos_to_oct_pos(pos)){
            Some(chunk.get_type(pos_to_local_pos(pos))?) 
        } else {
            let chunk = self.all_chunks.get_chunk_by_position(pos_to_oct_pos(pos))?;
            todo!()
        }
    }

    fn msg_voxel(&self, pos: Pos, msg: VoxelMsg<Vox>) {
        if let Some(chunk) = self.loaded_chunks.get_chunk_by_position(pos_to_oct_pos(pos)){
            chunk.tell(pos_to_local_pos(pos), msg)
        }
    }

    fn load(&mut self, pos: &[Pos]) {
        todo!()
    }

    fn read_phase<'v, V>(&self, registry: &V) where V: VisitorRegistry<'v, Vox, Self> {
        todo!()
    }

    fn respond_phase<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, Vox, Self> {
        todo!()
    }

    fn apply_phase<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, Vox, Self> {
        todo!()
    }
}

impl<Vox> Default for Map<Vox>
where 
Vox: core_obj::Voxel + Send + Sync, 
{
    fn default() -> Self {
        Self { 
            loaded_chunks: OctTree::<Chunk<Vox>, OctVec<u32>>::new(),
            all_chunks:  OctTree::<(), OctVec<u32>>::new(),
        }
    }
}
//DEPENDENCY INVERSION
pub trait ChunkTrait<Vox> 
where 
Vox: core_obj::Voxel + Send + Sync, 
{
    fn get_type(&self, pos: LocalPos) -> Option<Vox::Type>;

    fn tell(&self, pos: LocalPos, msg: VoxelMsg<Vox>);

    fn read_phase<'a, V>(&self, registry: &V) where V: VisitorRegistry<'a, Vox,Map<Vox>>;

    fn respond_phase<'a, V>(&mut self, registry: &V) where V: VisitorRegistry<'a, Vox,Map<Vox>>;

    fn apply_phase<'a, V>(&mut self, registry: &V) where V: VisitorRegistry<'a, Vox,Map<Vox>>;

    fn new() -> Self;
}