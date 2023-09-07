use crate::types::{Type, Instance};

/// An 'entity' that exists in a dimension that is not confined to the grid.
/// It is not a voxel, but it does has position and some shared components.
/// I'm not using the name entity to avoid confusion with the "E" in ECS.
#[derive(Debug)]
pub struct Element {
    pub my_type: i16,
}

pub trait ElementType: Type<Element, ElementInstancePtr> {

}
static_trait_ptr!(ElementType);

pub trait ElementInstance: Instance<Element> {

}
dynamic_static_trait_ptr!(ElementInstance);