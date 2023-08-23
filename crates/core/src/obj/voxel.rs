
use sc_prelude::Debug;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Voxel{
    pub my_type: VoxelTypePtr,
}


pub trait VoxelType: Type<Voxel>{

}

dyn_clone::clone_trait_object!(VoxelType);
#[derive(Debug, Clone)]
pub enum VoxelTypePtr{
    Static(&'static dyn VoxelType),
    Dyn(Box<dyn VoxelType>)
}