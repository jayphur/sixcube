use crate::types::Type;


/// An 'entity' that exists in a dimension that is not confined to the grid. 
/// It is not a voxel, but it does has position and some shared components.
/// I'm not using the name entity to avoid confusion with the "E" in ECS.
#[derive(Debug)]
pub struct Element{
    pub my_type: i16,
}

pub trait ElementType: Type<Element>{
    
}

dyn_clone::clone_trait_object!(ElementType);
#[derive(Debug, Clone)]
pub enum ElementTypePtr{
    Static(&'static dyn ElementType),
    Dyn(Box<dyn ElementType>)
}