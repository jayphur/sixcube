use crate::{types::ObjType, component::display::Displayable};
use sc_prelude::Debug;

#[derive(Debug, Clone)]
pub struct Voxel {
    pub my_type: VoxelTypePtr,
}
pub trait VoxelType: ObjType<Voxel> + Displayable {
    fn new(&self) -> VoxelTypePtr;
}
trait_ptr_enum!(VoxelType);