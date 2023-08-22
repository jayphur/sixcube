use crate::{Seed, obj::{dim::{DimType, Dim, MapTrait}, voxel::Voxel}, pos::GlobalPos, r#type::StaticType, map::Map};
use sc_prelude::*;


#[derive(Debug)]
pub struct DimTypePtr(&'static dyn DimType);
impl DimType for DimTypePtr{
    fn gen_at(&self, seed: Seed, pos: GlobalPos) -> Voxel {
        todo!()
    }
}
impl StaticType for DimTypePtr{
    type Obj = Dim;
    fn name(&'static self) -> &crate::Name {
        todo!()
    }
    fn new_obj(&'static self) -> Self::Obj {
        Dim{
            my_type: DimTypePtr(self.0), 
            map:  Map::new(),
        }
    }
}