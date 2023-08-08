use sc_core::obj::dim;
use sc_prelude::*;

use crate::gen::Gen;

use self::chunk_space::ChunkSpace;

mod chunk_space;
mod chunk;

#[derive(Debug, Default)]
pub struct Map<V,E> where
V: crate::Voxel,
E: crate::Entity, 
{
    voxels: ChunkSpace<V>,
    _e: PhantomData<E>
}
impl<V,E> dim::MapTrait<V,E> for Map<V,E>where
V: crate::Voxel,
E: crate::Entity, 
{
    type Gen = Gen<V, E>;
}