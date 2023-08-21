use sc_prelude::*;
use crate::{obj::{dim, voxel::Voxel}, pos::GlobalPos, Seed};
use self::chunk_space::ChunkSpace;

mod chunk_space;

#[derive(Debug, Default)]
pub struct Map<V,E> where
V: Debug + Default,
E: Debug,
{
    voxels: ChunkSpace<V>,
    _e: PhantomData<E>
}
impl<V,E> dim::MapTrait for Map<V,E> where
V: Debug + Default,
E: Debug,
{
    fn new() -> Self {
        todo!()
    }

    fn set_seed(&mut self, seed: Seed) {
        todo!()
    }

    fn get(&self, pos: GlobalPos) -> Option<&Voxel> {
        todo!()
    }

    fn get_mut_weak(&mut self, pos: GlobalPos, dim: &dim::Dim) -> Option<&mut Voxel> {
        todo!()
    }

    fn generate_region<D: dim::DimType>(&mut self, pos: GlobalPos, dim: &D) -> &mut Result<()> {
        todo!()
    }
}