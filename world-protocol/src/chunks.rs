use core_obj::Runtime;

use crate::pos::ChunkLocalPos;

pub trait ReadChunk<R:Runtime>{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<R::VoxelType>;
    fn get_data(&self, pos: ChunkLocalPos) -> Option<&R::DataContainer>;
}
pub trait WriteChunk<R:Runtime>: ReadChunk<R>{
    fn get_type_mut(&mut self, pos: ChunkLocalPos) -> &mut Option<R::VoxelType>;
    fn get_data_mut_weak(&mut self, pos: ChunkLocalPos) -> Option<&mut R::DataContainer>;
    fn get_data_mut(&mut self, pos: ChunkLocalPos) -> &mut R::DataContainer;
}