use crate::{Seed, obj::{dim::{DimType, Dim}, voxel::Voxel}, pos::GlobalPos, r#type::Type};
use sc_prelude::*;


#[derive(Debug)]
pub struct DimTypePtr(&'static dyn DimType);
impl DimType for DimTypePtr{
    fn gen_at(&self, seed: Seed, pos: GlobalPos) -> Result<Voxel> {
        todo!()
    }
}
impl Type for DimTypePtr{
    type Obj = Dim;
    fn name(&'static self) -> &crate::Name {
        todo!()
    }
    fn new_obj(&'static self) -> Self::Obj {
        Dim{my_type: DimTypePtr(self.0)}
    }
}