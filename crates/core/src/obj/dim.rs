use super::{element::Element, voxel::Voxel};
use crate::{data::pos::GlobalPos, map::Map, types::ObjType, Seed};
use sc_prelude::*;

/// Dimension. Dimension wide that/Dimension specific data.
#[derive(Debug)]
pub struct Dim {
    pub my_type: DimTypeTypePtr,
    pub map: Map<Voxel, Element>,
}
impl Dim {}

///The requirements that a DimType (ptr) must be able to do
pub trait DimType: ObjType<Dim> {
    fn gen(&self, seed: Seed, pos: GlobalPos) -> Option<Voxel>;
}
/// A map stores the voxels/chunks(?) in a dimension.
/// This is the data structure that holds the voxels.
pub trait MapTrait: Debug {
    fn new() -> Self;
    /// The seed is data that is guaranteed to be used in every dim.
    fn set_seed(&mut self, seed: Seed);
    /// Get this voxel if you can.
    fn get(&self, pos: GlobalPos) -> Result<Option<&Voxel>, MapError>;
    /// Get this voxel mutably if you can.
    fn get_mut_weak(&mut self, pos: GlobalPos) -> Result<Option<&mut Voxel>, MapError>;
    /// Load stuff thats gotta be loaded.
    fn load(&mut self, dim: &DimTypeTypePtr) -> Result<()>;
    /// Generate stuff stuff thats gotta be generated.
    fn gen(&mut self, dim: &DimTypeTypePtr) -> Result<()>;
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

dyn_clone::clone_trait_object!(DimType);
#[derive(Debug, Clone)]
pub enum DimTypeTypePtr {
    Static(&'static dyn DimType),
    Dyn(Box<dyn DimType>),
}

impl DimType for DimTypeTypePtr{
    fn gen(&self, seed: Seed, pos: GlobalPos) -> Option<Voxel> {
        match self{
            DimTypeTypePtr::Static(p) => p.gen(seed,pos),
            DimTypeTypePtr::Dyn(p) => p.gen(seed,pos),
        }
    }
}
impl ObjType<Dim> for DimTypeTypePtr{
    fn name(&self) -> &crate::Name {
        match self{
            DimTypeTypePtr::Static(p) => p.name(),
            DimTypeTypePtr::Dyn(p) => p.name(),
        }
    }

    fn is_master(&self) -> bool {
        match self{
            DimTypeTypePtr::Static(p) => p.is_master(),
            DimTypeTypePtr::Dyn(p) => p.is_master(),
        }
    }

    fn new_obj(&self) -> Dim {
        match self{
            DimTypeTypePtr::Static(p) => p.new_obj(),
            DimTypeTypePtr::Dyn(p) => p.new_obj(),
        }
    }
}