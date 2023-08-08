use std::{fmt::Debug};
use sc_prelude::*;
use crate::{data::pos::GlobalPos, r#type::DimType};

use super::{voxel::Voxel, element::Element};

/// Dimension
pub struct Dim<Map> where
Map: MapTrait<Voxel, Element> + 'static,
{
    my_type: &'static dyn DimType<Map>,
    map: Map,
}

/// A map stores the voxels/chunks(?) in a dimension
pub trait MapTrait<V, E>: Default + Debug{
    type Gen: MapGen<V, E, Self>;
}
/// Generates a map (something that impl MapTrait<V,E>)
pub trait MapGen<V, E, M>: Debug + Default + Clone where
M: MapTrait<V, E>,
{
    type Seed: Clone;

    fn set_seed(&mut self, seed: &Self::Seed) -> Result<()>;
    fn generate_at(&self, map: &mut M, pos: GlobalPos) -> Result<()>;
    fn generate_at_rad(&self, map: &mut M, center: GlobalPos, radius: u16)  -> Result<()>;
}
