use std::marker::PhantomData;

use core_obj::{Pos, Voxel, Runtime};
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
pub struct Map<R> 
where 
R: Runtime, 
{
    _marker: PhantomData<R>
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

    fn read_phase<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, R, Self>{
        todo!()
    }

    fn respond_phase<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, R, Self> {
        todo!()
    }

    fn apply_phase<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, R, Self> {
        todo!()
    }
}
impl<R> Default for Map<R> 
where 
R: Runtime, 
{
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
//DEPENDENCY INVERSION
pub trait ChunkTrait<R> 
where 
R: Runtime, 
{
    fn get_type(&self, pos: LocalPos) -> Option<R::VoxelType>;

    fn tell(&self, pos: LocalPos, msg: VoxelMsg<R>);

    fn read_phase<'a, V>(&self, registry: &V, map: &Map<R>) where V: VisitorRegistry<'a, R,Map<R>>;

    fn respond_phase<'a, V>(&mut self, registry: &V) where V: VisitorRegistry<'a, R,Map<R>>;

    fn apply_phase<'a, V>(&mut self, registry: &V) where V: VisitorRegistry<'a, R,Map<R>>;

    fn new(cw_pos: Pos) -> Self;
}