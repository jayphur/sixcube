use crate::{ecs::ComponentList, r#type::VoxelType};

#[derive(Debug)]
pub struct Voxel{
    pub my_type: &'static dyn VoxelType,
    // example for a mandatory component: pub comp_thing: Thing 
    pub comp_opt: Option<ComponentList>, //honestly we might remove this
}