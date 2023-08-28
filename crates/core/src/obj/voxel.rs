use crate::types::ObjType;
use sc_prelude::Debug;

#[derive(Debug, Clone)]
pub struct Voxel {
    pub my_type: VoxelTypePtr,
}
pub trait VoxelType: ObjType<Voxel> {

}
trait_ptr_enum!(VoxelType);
