use crate::r#type::Type;

pub mod dim;
pub mod element;
pub mod voxel;
pub mod world;
pub mod type_ptr{
    pub mod dim;
    pub mod element;
    pub mod voxel;
}
trait Obj{
    type Type: Type<Obj = Self>;
}