use core_obj::Registrar;

use crate::pos::ChunkLocalPos;

pub trait ReadChunk<R: Registrar>{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<R::VoxelType>;
    fn get_data(&self, pos: ChunkLocalPos) -> Option<&R::DataContainer>;
}

///Writing to chunk, sending events if necessary.
pub trait WriteChunk<R: Registrar>: ReadChunk<R>{
    fn get_type_mut(&mut self, pos: ChunkLocalPos) -> &mut Option<R::VoxelType>;
    fn get_data_mut_weak(&mut self, pos: ChunkLocalPos) -> Option<&mut R::DataContainer>;
    fn get_data_mut(&mut self, pos: ChunkLocalPos) -> &mut R::DataContainer;
}