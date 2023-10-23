#![feature(return_position_impl_trait_in_trait)]

use core_obj::*;
use prelude::*;
use visit::{Message, VoxelVisitor};

pub mod visit;

pub trait Map<T, D, M>: Debug
where
    T: TypeId,
    D: Data,
    M: Message,
    Self: Sized,
{
    fn get_type(&self, pos: Pos) -> Option<T>;
    fn tell(&self, pos: Pos, msg: M);
    /// Iter LOADED voxels
    fn visit_each<'v, V>(&self, visitors: &'v [V])
    where V: 'v + Send + Sync + VoxelVisitor<T,D,M, Self>;
    /// Iter LOADED voxels
    fn visit_each_mut<'v, V>(&mut self, visitors: &'v [V])
    where V: 'v + Send + Sync + VoxelVisitor<T,D,M, Self>;
}

#[derive(Debug, Clone, Copy, Default)]
pub enum IsLoaded {
    #[default]
    Loaded,
    UnLoaded,
    All,
}
