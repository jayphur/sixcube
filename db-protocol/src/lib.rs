#![feature(return_position_impl_trait_in_trait)]

use core_obj::*;
use message::Message;
use prelude::*;

mod update;
pub mod message;

pub trait Map<'a, T, D>: Debug 
where T: TypeId, D: Data, Self: Sized {
    type VoxelIter : VoxelIter<'a, T,D> + Send ;
    fn get_type(&self, pos: Pos) -> Option<T>;
    fn tell(&self, pos: Pos, msg: Message<T,D>);
    /// Iter LOADED voxels
    fn iter_voxels(&'a self) -> Self::VoxelIter;
    /// Iter LOADED voxels
    fn for_each_voxel<F>(&mut self, f: F)
    where F: Fn(&mut Option<Voxel<T,D>>, Pos) -> () + Sync + Send;
}

pub trait VoxelIter<'a, T: TypeId, D: Data>{
    fn for_each<F>(&mut self, f: F)
    where F: Fn(&Option<Voxel<T,D>>, &Pos) -> () + Sync + Send;
}
#[derive(Debug, Clone, Copy, Default)]
pub enum IsLoaded{
    #[default]
    Loaded,
    UnLoaded,
    All,
}
