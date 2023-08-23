use crate::{
    obj::{dim, voxel::Voxel},
    pos::GlobalPos,
    Seed,
};
use sc_prelude::*;

use self::{chunk::Chunk, octree::Octree};

mod chunk;
mod octree;

const CHUNK_SIZE: usize = 16;

#[derive(Debug, Default)]
pub struct Map<V, E>
where
    V: Debug + Default + Clone,
    E: Debug,
{
    voxels: Octree<Option<Chunk<V, CHUNK_SIZE>>>,
    seed: Seed,
    _e: PhantomData<E>,
}
impl<V, E> dim::MapTrait for Map<V, E>
where
    V: Debug + Default + Clone,
    E: Debug,
{
    fn new() -> Self {
        todo!()
    }

    fn set_seed(&mut self, seed: Seed) {
        self.seed = seed;
    }

    fn get(&self, pos: GlobalPos) -> Option<&Voxel> {
        let chunk = self.voxels.get_weak(pos.tuple())?.as_ref()?;
        todo!()
    }

    fn get_mut_weak(&mut self, pos: GlobalPos, dim: &dim::Dim) -> Option<&mut Voxel> {
        todo!()
    }

    fn generate_region<D: dim::DimType>(&mut self, pos: GlobalPos, dim: &D) -> &mut Result<()> {
        todo!()
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
