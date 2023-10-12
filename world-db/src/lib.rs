#![feature(return_position_impl_trait_in_trait)]
use core_obj::*;
use db_protocol::update::Message;
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
    fn iter_voxel<'a>(&'a self, cw_pos: CwPos) -> impl Iterator<Item = (&Option<Voxel<T, D>>, Pos)>
    where
        D: 'a,
        T: 'a;
    fn iter_voxel_mut<'a>(
        &'a mut self,
        cw_pos: CwPos,
    ) -> impl Iterator<Item = (&mut Option<Voxel<T, D>>, Pos)>
    where
        D: 'a,
        T: 'a;
}

impl<'a, T: TypeId + 'a, D: Data + 'a, M: Message + 'a> db_protocol::Map<'a, T, D, M>
    for Map<T, D, M>
{
    type VoxelIter = VoxelIter<'a, T, D, M>;

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

    fn iter_voxels(&'a self) -> Self::VoxelIter {
        VoxelIter { map: self }
    }

    /// Run a closure on each voxel of `&mut Voxel<...>` and `Pos`
    ///
    /// ...where `Pos` is that voxel's position.
    fn for_each_voxel<F>(&mut self, f: F)
    where
        F: Fn(&mut Option<Voxel<T, D>>, Pos) -> () + Sync + Send,
    {
        self.loaded_chunks.iter().for_each(|cw_pos| {
            if let Some(index) = self.tree.find_index(cw_pos) {
                self.to_update_cw_pos.push(*cw_pos);
                self.to_update_memory_index.push(index);
            }
        });
        self.tree
            .get_raw_many_mut(&self.to_update_memory_index)
            .into_par_iter()
            .zip(self.to_update_cw_pos.par_iter())
            .for_each(|(chunk, cw_pos)| {
                chunk
                    .iter_voxel_mut(*cw_pos)
                    .for_each(|(vox, pos)| f(vox, pos))
            });
        self.to_update_cw_pos.clear();
        self.to_update_memory_index.clear();
    }
}

pub struct VoxelIter<'a, T, D, M>
where
    T: TypeId + 'a + Send + Sync,
    D: Data + 'a + Send + Sync,
    M: Message,
{
    map: &'a Map<T, D, M>,
}
impl<T, D, M> db_protocol::VoxelIter<'_, T, D, M> for VoxelIter<'_, T, D, M>
where
    T: TypeId + Send + Sync,
    D: Data + Send + Sync,
    M: Message,
{
    fn for_each<F>(&mut self, f: F)
    where
        F: Fn(&Option<Voxel<T, D>>, &Pos) -> () + Sync + Send,
    {
        self.map.loaded_chunks.par_iter().for_each(|cw_pos| {
            if let Some(chunk) = self.map.tree.get_weak(cw_pos) {
                chunk
                    .iter_voxel(*cw_pos)
                    .for_each(|(vox, pos)| f(vox, &pos))
            }
        });
    }
}
