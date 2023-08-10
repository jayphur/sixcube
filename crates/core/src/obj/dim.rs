use sc_prelude::*;
use crate::{data::pos::GlobalPos, map, r#type::Type, Seed};

use super::{voxel::Voxel, Obj, type_ptr::dim::DimTypePtr};

/// Dimension. Dimension wide that/Dimension specific data.
#[derive(Debug)]
pub struct Dim{
    pub my_type: DimTypePtr,
}
impl Obj for Dim{
    type Type = DimTypePtr;
}
impl map::DimTrait for Dim{
    fn gen_at(&self, seed: Seed, pos: GlobalPos) -> Result<Voxel> {
        self.my_type.gen_at(seed, pos)
    }
}



///The requirements that a DimType (ptr) must be able to do 
pub trait DimType: Type<Obj=Dim>{
    fn gen_at(&self, seed: Seed, pos: GlobalPos) -> Result<Voxel>;
}