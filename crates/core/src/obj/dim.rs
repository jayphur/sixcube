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
    type Gen: MapGen<V, E, Self>;
}
pub trait MapGen<V, E, M>: Debug + Default + Clone where
M: MapTrait<V, E>,
{
    type Seed: Clone;

    fn set_seed(&mut self, seed: &Self::Seed) -> Result<()>;
    fn generate_at(&self, map: &mut M, pos: GlobalPos) -> Result<()>;
    fn generate_at_rad(&self, map: &mut M, center: GlobalPos, radius: u16)  -> Result<()>;
}