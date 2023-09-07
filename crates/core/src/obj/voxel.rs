use crate::{types::{Type, Instance}, display::map::VoxelDisplayInfo};
use sc_prelude::Debug;

#[derive(Debug, Clone)]
pub struct Voxel {
    pub my_type: VoxelInstancePtr,
}
pub trait VoxelType: Type<Voxel, VoxelInstancePtr>  {

}
static_trait_ptr!(VoxelType);

pub trait VoxelInstance: Instance<Voxel> + VoxelDisplayInfo{

}
dynamic_static_trait_ptr!(VoxelInstance);