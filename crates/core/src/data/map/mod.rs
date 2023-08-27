use std::collections::VecDeque;

use crate::{
    obj::{
        dim::{self, MapError, DimTypeTypePtr},
        element::Element,
        voxel::Voxel,
    },
    pos::{GlobalPos, RelativePos},
    Seed,
};
use sc_prelude::*;
use self::{chunk::Chunk, octree::Octree};
use crate::CHUNK_SIZE;

mod chunk;
mod octree;

#[derive(Debug, Default)]
pub struct Map<V, E>
where
    V: Debug + Clone,
    E: Debug,
{
    chunks: Octree<Chunk<Option<V>, CHUNK_SIZE>>,
    seed: Seed,
    to_generate: VecDeque<GlobalPos>,
    to_loading: VecDeque<GlobalPos>,
    _e: PhantomData<E>,
}
impl dim::MapTrait for Map<Voxel, Element> {
    fn new() -> Self {
        todo!()
    }

    fn set_seed(&mut self, seed: Seed) {
        self.seed = seed;
    }

    fn get(&self, pos: GlobalPos) -> Result<Option<&Voxel>, MapError> {
        todo!()
    }

    fn get_mut_weak(&mut self, pos: GlobalPos) -> Result<Option<&mut Voxel>, MapError> {
        todo!()
    }

    fn load(&mut self, dim: &DimTypeTypePtr) -> Result<()> {
        todo!()
    }

}
impl<V, E> Map<V, E>
where
    V: Debug + Default + Clone,
    E: Debug,
{

}
impl<E: Debug> Map<Voxel,E>{
    fn generate_chunk(&mut self, dim: &DimTypeTypePtr, pos: GlobalPos) -> Result<()>{
        let chunk_pos = pos.chunk();
        let chunk = self.chunks.get_mut_strong(chunk_pos);
        match dim{
            DimTypeTypePtr::Static(d) => {
                for &relative in Chunk::<Option<Voxel>, CHUNK_SIZE>::all_pos(){
                    *chunk.get_mut(relative)? = 
                        d.gen_at(self.seed, GlobalPos::new_from_parts(chunk_pos, relative));                
                    }
            },
            DimTypeTypePtr::Dyn(d) => {
                for &relative in Chunk::<Option<Voxel>, CHUNK_SIZE>::all_pos(){
                    *chunk.get_mut(relative)? = 
                        d.gen_at(self.seed, GlobalPos::new_from_parts(chunk_pos, relative));
                }
            },
        }
        
        Ok(())
    }
}

trait OctreeTrait<T: Default + Debug>: Debug + Default {
    fn new(size: u16) -> Self;
    fn get_weak(&self, pos: (i16, i16, i16)) -> Option<&T>;
    /// Will not create a new one if this position doesn't exist.
    fn get_mut_weak(&mut self, pos: (i16, i16, i16)) -> Option<&mut T>;
    /// Will create a new one if this position doesn't exist.
    fn get_mut_strong(&mut self, pos: (i16, i16, i16)) -> &mut T;
}

trait ChunkTrait<T: Default + Debug + Clone + Sized>: Debug + Default {
    fn new() -> Self;
    fn get(&self, pos: RelativePos) -> Result<&T>;
    fn get_mut(&mut self, pos: RelativePos) -> Result<&mut T>;
    fn all_pos() -> &'static Vec<RelativePos>;
}
