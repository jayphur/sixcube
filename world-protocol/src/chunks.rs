use core_obj::{DataContainer, VoxelId};

use crate::pos::ChunkLocalPos;

pub trait ReadChunk{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<VoxelId>;
    fn get_data(&self, pos: ChunkLocalPos) -> Option<&DataContainer>;
}

///Writing to chunk, sending events if necessary.
pub trait WriteChunk: ReadChunk{
    fn get_type_mut(&mut self, pos: ChunkLocalPos) -> &mut Option<VoxelId>;
    fn get_data_mut_weak(&mut self, pos: ChunkLocalPos) -> Option<&mut DataContainer>;
    fn get_data_mut(&mut self, pos: ChunkLocalPos) -> &mut DataContainer;
}