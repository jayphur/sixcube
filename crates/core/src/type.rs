use std::fmt::Debug;
use crate::data::Name;

/// Type of something. a block / item / etc. Can be applied to "obj"s.
/// Exists statically for all obj's that point to it
pub trait StaticType: Debug{
    type Obj: Debug;
    
    fn name(&'static self) -> &Name;
    fn new_obj(&'static self) -> Self::Obj;
}

/// Exists on a dynamic, per-object, basis.
/// Can be applied to "obj"s like a static type, but with unique data for each object.
/// It still has a "static type" however.
pub trait DynType: Debug{
    type Obj: Debug;
    fn my_type(&self) -> &'static dyn StaticType<Obj=Self::Obj>;
}