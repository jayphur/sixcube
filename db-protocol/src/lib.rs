#![feature(return_position_impl_trait_in_trait)]

use core_obj::*;
use prelude::*;
use update::Message;

pub mod update;

pub trait Map<'a, T, D, M>: Debug 
where 
T: TypeId, 
D: Data, 
M: Message,
Self: Sized 
{
    type VoxelIter : VoxelIter<'a, T,D,M> + Send ;
    fn get_type(&self, pos: Pos) -> Option<T>;
    fn tell(&self, pos: Pos, msg: M);
    /// Iter LOADED voxels
    fn iter_voxels(&'a self) -> Self::VoxelIter;
    /// Iter LOADED voxels
    fn for_each_voxel<F>(&mut self, f: F)
    where F: Fn(&mut Option<Voxel<T,D>>, Pos) -> () + Sync + Send;
}

pub trait VoxelIter<'a, T: TypeId, D: Data, M: Message>{
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
