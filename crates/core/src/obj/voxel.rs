use crate::{types::{Type, TypeInstance}, display::dim::VoxelDisplayInfo};
use sc_prelude::Debug;

#[derive(Debug, Clone)]
pub struct Voxel {
    pub my_type: VoxelTypePtr,
}
pub trait VoxelType: Type<Voxel, VoxelTypeInstancePtr>  {

}
static_trait_ptr!(VoxelType);

pub trait VoxelTypeInstance: TypeInstance<Voxel> + VoxelDisplayInfo{

}
dynamic_static_trait_ptr!(VoxelTypeInstance);
