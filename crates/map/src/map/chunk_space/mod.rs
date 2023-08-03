use sc_prelude::*;

use super::chunk::Chunk;

mod octree;

#[derive(Debug, Default)]
pub(crate) struct ChunkSpace<V: Debug + Default>{
    grid: octree::Octree<Chunk<V, 16>>
}

trait GrowingOctree<T>: Debug + Default{
    fn get_weak(&self, pos: (i16,i16,i16)) -> Option<&T>;
    /// Will not create a new one if this position doesn't exist.
    fn get_mut_weak(&mut self, pos: (i16,i16,i16)) -> Option<&mut T>;
    /// Will create a new one if this position doesn't exist.
    fn get_mut_strong(&mut self, pos: (i16,i16,i16)) -> &mut T;
}