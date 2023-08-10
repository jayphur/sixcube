use sc_prelude::*;


use self::chunk_space::ChunkSpace;

mod chunk_space;
mod chunk;

#[derive(Debug, Default)]
pub struct Map<V: Debug + Default,E>{
    voxels: ChunkSpace<V>,
    _e: PhantomData<E>
}