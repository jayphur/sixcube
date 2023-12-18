use core_obj::{Attr, TypeInfo, Voxel};

#[derive(Debug, Clone)]
pub struct Runtime{

}
impl core_obj::Runtime for Runtime{
    type VoxelType = u32;
    fn get_voxels(&self) -> &[Self::VoxelType] {
        todo!()
    }
    fn voxel_default(&self, r#type: &Self::VoxelType) -> &Voxel<Self> {
        todo!()
    }
    fn voxel_info(&self, r#type: &Self::VoxelType) -> &TypeInfo {
        todo!()
    }

    type AttrType = u32;
    fn get_attr(&self) -> &[Self::AttrType] {
        todo!()
    }
    fn attr_default(&self, r#type: &Self::AttrType) -> &Attr<Self> {
        todo!()
    }
    fn attr_info(&self, r#type: &Self::AttrType) -> &TypeInfo {
        todo!()
    }
}