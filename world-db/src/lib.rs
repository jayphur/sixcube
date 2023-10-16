#![feature(return_position_impl_trait_in_trait)]
use core_obj::*;
use db_protocol::visit::{Message, VoxelVisitor};
use octree::Octree;
use prelude::*;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

mod chunk;
mod octree;

const CHUNK_SIZE: usize = 4;
const CHUNK_SIZE_I32: i32 = 4;

// Chunk-Wise pos
type CwPos = Pos;

#[derive(Debug)]
pub struct Map<T: TypeId, D: Data, M: Message> {
    tree: Octree<chunk::Chunk<T, D, M>>,
    loaded_chunks: Vec<CwPos>,
    to_update_memory_index: Vec<usize>,
    to_update_cw_pos: Vec<CwPos>,
}

/// DEPENDENCY INVERSION
trait OctreeTrait<T: Default + Debug>: Debug + Default + Send {
    fn new(size: u16) -> Self;
    fn get_weak(&self, pos: &CwPos) -> Option<&T>;
    /// Will not create a new one if this position doesn't exist.
    fn get_mut_weak(&mut self, pos: &CwPos) -> Option<&mut T>;
    /// Will create a new one if this position doesn't exist.
    fn get_mut_strong(&mut self, pos: &CwPos) -> &mut T;
    fn find_index(&self, pos: &CwPos) -> Option<usize>;
    fn get_raw(&self, index: usize) -> Option<&T>;
    fn get_raw_mut(&mut self, index: usize) -> Option<&mut T>;
    fn slice_raw(&self) -> &[T];
    fn slice_raw_mut(&mut self) -> &mut [T];
    fn get_raw_many_mut(&mut self, many: &Vec<usize>) -> Vec<&mut T>;
}
trait ChunkTrait<T: TypeId, D: Data, M: Message>: Debug + Default + Send {
    fn contains_attr(&self, attr: T::AttrId) -> bool;
    /// Not Cw, relative.
    fn tell(&self, pos: Pos, msg: M);
    /// Not Cw, relative.
    fn get(&self, pos: Pos) -> &Option<Voxel<T, D>>;
    /// Not Cw, relative.
    fn get_mut(&mut self, pos: Pos) -> &mut Option<Voxel<T, D>>;
}

impl<'a, T: TypeId + 'a, A: AttrId + 'a, D: Data + 'a, M: Message + 'a> db_protocol::Map<'a, T, A, D, M>
    for Map<T, D, M>
{
    fn get_type(&self, pos: Pos) -> Option<T> {
        Some(
            self.tree
                .get_weak(&(pos / CHUNK_SIZE as i32))?
                .get(pos)
                .as_ref()?
                .type_id,
        )
    }

    fn tell(&self, pos: Pos, msg: M) {
        let Some(chunk) = self.tree.get_weak(&(pos / CHUNK_SIZE as i32)) else {return ();};
        chunk.tell(pos, msg);
    }

    fn do_each_visitor(&self, visitors: &[&dyn VoxelVisitor<T,A,D,M, Self>]) {
        todo!()
    }

    fn do_each_visitor_mut(&mut self, visitors: &[&dyn VoxelVisitor<T,A,D,M, Self>]) {
        todo!()
    }

}
