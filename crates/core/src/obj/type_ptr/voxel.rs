use crate::{obj::voxel::{Voxel, VoxelType}, r#type::Type};

#[derive(Debug, Clone)]
pub struct VoxelTypePtr(&'static dyn VoxelType);
impl VoxelType for VoxelTypePtr{

}
impl Type for VoxelTypePtr{
    type Obj = Voxel;

    fn name(&'static self) -> &crate::Name {
        todo!()
    }

    fn new_obj(&'static self) -> Self::Obj {
        Voxel{
            my_type: VoxelTypePtr(self.0),
            comp_opt: None,
        }
    }
}