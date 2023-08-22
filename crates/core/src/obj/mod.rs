use crate::r#type::StaticType;

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
    type Type: StaticType<Obj = Self>;
}