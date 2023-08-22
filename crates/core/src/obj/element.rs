use crate::{ecs::ComponentList, r#type::StaticType};

use super::{Obj, type_ptr::element::ElementTypePtr};

/// An 'entity' that exists in a dimension that is not confined to the grid. 
/// It is not a voxel, but it does has position and some shared components.
/// I'm not using the name entity to avoid confusion with the "E" in ECS.
#[derive(Debug)]
pub struct Element{
    pub my_type: ElementTypePtr,
    pub comp_opt: Option<ComponentList>, //honestly we might remove this
}
impl Obj for Element{
    type Type = ElementTypePtr;
}


pub trait ElementType: StaticType<Obj=Element>{
    
}
