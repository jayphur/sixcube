use std::fmt::Debug;
use crate::data::Name;

/// Type of something. a block / item / etc. Can be applied to "obj"s.
pub trait Type: Debug{
    type Obj: Debug;
    
    fn name(&'static self) -> &Name;
    fn new_obj(&'static self) -> Self::Obj;
}

// the type trait ready for specific objects. They should be able to return some info with using a `&'static self`

pub trait ContainsTypes<'a, TypePtr: Clone>{
    type Id;
    fn get(&'a self, id: Self::Id) -> TypePtr; 
    fn deref_to_id(ptr: TypePtr) -> Self::Id;
}