use sc_prelude::Debug;

use crate::{r#type::{StaticType, DynType}};
use super::type_ptr::voxel::VoxelTypePtr;

#[derive(Debug, Clone)]
pub struct Voxel{
    pub my_type: VoxelTypePtr
}

pub trait VoxelStaticType: StaticType<Obj=Voxel>{

}
pub trait VoxelDynType: DynType<Obj=Voxel>{
    fn clone_to_box(&self) -> Box<dyn VoxelDynType<Obj=Self::Obj>>;
}

impl Default for Voxel{
    fn default() -> Self {
        todo!()
    }
}

