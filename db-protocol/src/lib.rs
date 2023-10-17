#![feature(return_position_impl_trait_in_trait)]

use core_obj::*;
use prelude::*;
use visit::{Message, VoxelVisitor};

pub mod visit;

pub trait Map<'a, T, D, M>: Debug
where
    T: TypeId,
    D: Data,
    M: Message,
    Self: Sized,
{
    fn get_type(&self, pos: Pos) -> Option<T>;
    fn tell(&self, pos: Pos, msg: M);
    /// Iter LOADED voxels
    fn do_each_visitor(&self, visitors: &[&dyn VoxelVisitor<T,D,M, Self>]);
    /// Iter LOADED voxels
    fn do_each_visitor_mut(&mut self, visitors: &[&dyn VoxelVisitor<T,D,M, Self>]);
}

#[derive(Debug, Clone, Copy, Default)]
pub enum IsLoaded {
    #[default]
    Loaded,
    UnLoaded,
    All,
}
