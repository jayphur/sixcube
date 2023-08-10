use crate::{ecs::ComponentList, r#type::Type};
use super::type_ptr::voxel::VoxelTypePtr;

#[derive(Debug)]
pub struct Voxel{
    pub my_type: VoxelTypePtr,
    // example for a mandatory component: pub comp_thing: Thing 
    pub comp_opt: Option<ComponentList>, //honestly we might remove this
}

pub trait VoxelType: Type<Obj=Voxel>{

}


impl Default for Voxel{
    fn default() -> Self {
        todo!()
    }
}

