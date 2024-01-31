use core_obj::Runtime;

use crate::pos::ChunkLocalPos;


pub trait ReadChunk<R:Runtime>{
    fn get_type(&self, pos: ChunkLocalPos) -> Option<R::VoxelType>;
    fn get_data(&self, pos: ChunkLocalPos) -> Option<()>; 
}
pub trait WriteChunk<R:Runtime>: ReadChunk<R>{
    fn set_type(&mut self, pos: ChunkLocalPos, r#type: R::VoxelType);
    fn set_data(&mut self, pos: ChunkLocalPos, data: ()); //TODO: what is data??? 
}