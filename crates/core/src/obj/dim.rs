use std::{fmt::Debug, marker::PhantomData};
use sc_prelude::*;
use super::pos::GlobalPos;

/// Dimension
pub struct Dim<Map, MapGen, Vox, Elem> where
Map: MapTrait<Vox, Elem>,
{
    map: Map,
    map_gen: MapGen,

    //markers
    voxel: PhantomData<Vox>,
    element: PhantomData<Elem>,
}

pub trait MapTrait<V, E>: Default + Debug{
    type MapProxy: MapProxy<V, E, Self>;
    type GenResult; //How should the generator return results?

    /// Should generate at least this area. if it doesn't exist.
    fn ensure_radius<G>(center: GlobalPos, radius: u16, world_gen: &G) -> Result<()> where
    G: MapGen<V, E, Self>;
}
pub trait MapProxy<V, E, M: MapTrait<V,E>>{

}
pub trait MapGen<V, E, M>: Debug + Default + Clone where
M: MapTrait<V, E>,
{
    type Seed: Clone;

    fn set_seed(&mut self, seed: &Self::Seed) -> Result<()>;
    //TODO: something to do with get(GlobalPos) -> ...
    fn generate_at(&self, pos: GlobalPos) -> M::GenResult;
}