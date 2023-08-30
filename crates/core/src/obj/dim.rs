use super::{element::Element, voxel::Voxel};
use crate::{data::pos::GlobalPos, map::Map, types::ObjType, Seed};
use async_trait::async_trait;
use sc_prelude::*;

/// Dimension. Dimension wide that/Dimension specific data.
#[derive(Debug)]
pub struct Dim {
    pub my_type: DimTypePtr,
    pub map: Map<Voxel, Element>,
}
impl Dim {}

///The requirements that a DimType (ptr) must be able to do
pub trait DimType: ObjType<Dim>{
    fn gen(&self, seed: Seed, pos: GlobalPos) -> Option<Voxel>;
    fn new(&self) -> DimTypePtr;
}
trait_ptr_enum!(DimType);


/// A map stores the voxels/chunks(?) in a dimension.
/// This is the data structure that holds the voxels.
#[async_trait]
pub trait MapTrait: Debug {
    fn new() -> Self;
    /// The seed is data that is guaranteed to be used in every dim.
    fn set_seed(&mut self, seed: Seed);
    /// Get this voxel if you can.
    fn get(&self, pos: GlobalPos) -> Result<&Option<Voxel>, MapError>;
    /// Get this voxel mutably if you can.
    fn get_mut_weak(&mut self, pos: GlobalPos) -> Result<&mut Option<Voxel>, MapError>;
    /// Get this voxel mutably, setting to load if not.
    fn get_mut_strong(&mut self, pos: GlobalPos) -> Result<&mut Option<Voxel>, MapError>;
    /// Load stuff thats gotta be loaded.
    async fn load(&mut self, dim: &DimTypePtr) -> Result<()>;
    /// Generate stuff stuff thats gotta be generated.
    fn gen(&mut self, dim: &DimTypePtr) -> Result<()>;
}


#[derive(thiserror::Error, Debug)]
pub enum MapError {
    #[error("Attempting to access loaded chunk, please generate the chunk.")]
    Unloaded,
    #[error("Attempting to access generated chunk, please generate the chunk.")]
    UnGenerated,
    #[error("Fatal error in map: {0}")]
    Fatal(Error),
}

