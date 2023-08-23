use super::{element::Element, voxel::Voxel};
use crate::{data::pos::GlobalPos, map::Map, types::Type, Seed};
use sc_prelude::*;

/// Dimension. Dimension wide that/Dimension specific data.
#[derive(Debug)]
pub struct Dim {
    pub my_type: i16,
    pub map: Map<Voxel, Element>,
}
impl Dim {}

///The requirements that a DimType (ptr) must be able to do
pub trait DimType: Type<Dim> {
    fn gen_at(&self, seed: Seed, pos: GlobalPos) -> Voxel;
}
/// A map stores the voxels/chunks(?) in a dimension.
/// This is the data structure that holds the voxels.
pub trait MapTrait: Debug {
    fn new() -> Self;
    /// The seed is data that is guaranteed to be used in every dim.
    fn set_seed(&mut self, seed: Seed);
    /// Get this voxel if you can.
    fn get(&self, pos: GlobalPos) -> Option<&Voxel>;
    /// Generate an area centered at this location using a `DimType`.
    /// Good thing to use in response `to get_mut_weak(...)` yielding `None`.
    fn generate_region<D: DimType>(&mut self, pos: GlobalPos, dim: &D) -> &mut Result<()>;
    /// Get this voxel mutably if you can.
    fn get_mut_weak(&mut self, pos: GlobalPos, dim: &Dim) -> Option<&mut Voxel>;
}

dyn_clone::clone_trait_object!(DimType);
#[derive(Debug, Clone)]
pub enum VoxelTypePtr {
    Static(&'static dyn DimType),
    Dyn(Box<dyn DimType>),
}
