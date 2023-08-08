use crate::{ecs::ComponentList, r#type::ElementType};

/// An 'entity' that exists in a dimension that is not confined to the grid. 
/// It is not a voxel, but it does has position and some shared components.
/// I'm not using the name entity to avoid confusion with the "E" in ECS.
pub struct Element{
    pub my_type: &'static dyn ElementType,
    pub comp_opt: Option<ComponentList>, //honestly we might remove this
}