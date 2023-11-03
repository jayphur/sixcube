#![feature(return_position_impl_trait_in_trait)]

use core_obj::*;
use prelude::*;
use visit::VoxelVisitor;

pub mod visit;

pub trait Map<Vox>: Debug
where
    Vox: Voxel,
    Self: Sized,
{
    type Msg: visit::Message;

    fn get_type(&self, pos: Pos) -> Option<Vox::Type>;
    fn tell(&self, pos: Pos, msg: Self::Msg);
    /// Iter LOADED voxels
    fn message_phase<'v, V>(&mut self, visitors: &'v [V])
    where V: 'v + Send + Sync + VoxelVisitor<Vox,Self>;
    /// Iter LOADED voxels
    fn respond_phase<'v, V>(&mut self, visitors: &'v [V])
    where V: 'v + Send + Sync + VoxelVisitor<Vox, Self>;
}

#[derive(Debug, Clone, Copy, Default)]
pub enum IsLoaded {
    #[default]
    Loaded,
    UnLoaded,
    All,
}