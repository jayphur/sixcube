use std::marker::PhantomData;

use core_obj::{Pos, Voxel};
use kdtree::KdTree;
use world_protocol::{message::VoxelMsg, VisitorRead, VisitorRespond, VisitorApply, VisitorRegistry};
use rayon::{prelude::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator}, iter::ParallelBridge};
use chunk::Chunk;

mod chunk;


// Tree tree tree tree tree!!!!!

//Smallest chunk 32^3
//nah maybe 8^3

type LocalPos = vector3d::Vector3d<usize>;

const CHUNK_SIZE: usize = 16;
const CHUNK_SIZE_I32: i32 = 16;

#[derive(Debug)]
pub struct Map<Vox> 
where 
Vox: core_obj::Voxel + Send + Sync, 
{
    _marker: PhantomData<Vox>
}
impl<Vox> world_protocol::Map<Vox> for Map<Vox> 
where 
Vox: core_obj::Voxel + Send + Sync, 
{
    fn get_type(&self, pos: Pos) -> Option<Vox::Type> {
        todo!()
    }

    fn msg_voxel(&self, pos: Pos, msg: VoxelMsg<Vox>) {
        todo!()

    }

    fn load(&mut self, pos: &[Pos]) {
        todo!()
    }

    fn read_phase<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, Vox, Self>{
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
            _marker: PhantomData,
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

    fn read_phase<'a, V>(&self, registry: &V, map: &Map<Vox>) where V: VisitorRegistry<'a, Vox,Map<Vox>>;

    fn respond_phase<'a, V>(&mut self, registry: &V) where V: VisitorRegistry<'a, Vox,Map<Vox>>;

    fn apply_phase<'a, V>(&mut self, registry: &V) where V: VisitorRegistry<'a, Vox,Map<Vox>>;

    fn new(cw_pos: Pos) -> Self;
}