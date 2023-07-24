use sc_core::obj::{dim, pos::GlobalPos};
use sc_prelude::*;

mod grid;
mod chunk;

#[derive(Debug, Default)]
pub struct Map<V,E> where
V: Debug + Default,
E: Debug + Default, 
{
    voxels: Vec<V>,
    _e: PhantomData<E>
}


impl<V, E>  dim::MapTrait<V, E> for Map<V,E> where
V: Debug + Default,
E: Debug + Default, 
{
    type MapProxy = MapProxy<V,E>;

    type GenResult = ();

    fn ensure_radius<G>(center: GlobalPos, radius: u16, world_gen: &G) -> Result<()> where
    G: dim::MapGen<V, E, Self> 
    {
        todo!()
    }
}


#[derive(Debug)]
pub struct MapProxy<V,E>{
    _v: PhantomData<V>,
    _e: PhantomData<E>,
}

impl<V,E> dim::MapProxy<V,E, Map<V,E>> for MapProxy<V, E> where
V: Debug + Default,
E: Debug + Default, {

}