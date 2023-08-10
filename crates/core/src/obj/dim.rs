use sc_prelude::*;
use crate::{data::pos::GlobalPos, r#type::Type, Seed, map::Map};
use super::{voxel::Voxel, Obj, type_ptr::dim::DimTypePtr, element::Element};

/// Dimension. Dimension wide that/Dimension specific data.
#[derive(Debug)]
pub struct Dim{
    pub my_type: DimTypePtr,
    pub map: Map<Voxel, Element>,
}
impl Obj for Dim{
    type Type = DimTypePtr;
}
impl Dim{
    fn gen_at(&self, seed: Seed, pos: GlobalPos) -> Result<Voxel> {
        self.my_type.gen_at(seed, pos)
    }
}



///The requirements that a DimType (ptr) must be able to do 
pub trait DimType: Type<Obj=Dim>{
    fn gen_at(&self, seed: Seed, pos: GlobalPos) -> Result<Voxel>;
}
/// A map stores the voxels/chunks(?) in a dimension. 
/// This is the data structure that holds the voxels. 
pub trait MapTrait: Debug{
    fn new() -> Self;
    fn set_seed(&mut self, seed: Seed);
    fn get(&self, pos: GlobalPos) -> Option<&Voxel>;
    fn get_mut(&mut self, pos: GlobalPos, dim: &Dim) -> &mut Option<Voxel>;
    fn get_mut_weak(&mut self, pos: GlobalPos, dim: &Dim) -> Option<&mut Voxel>;
}