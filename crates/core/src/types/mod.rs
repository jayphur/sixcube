use dyn_clone::DynClone;
use std::fmt::Debug;
use crate::data::TypeId;

mod list;
pub use list::TypePtrList;

/// Type of something. a block / item / etc. Can be applied to "obj"s.
/// This is the SINGLE static "master" variant of this trait.
/// Most likely the `TypeInstance`s will have some kind of pointer back to this master 
/// `Type`.
pub trait Type<Obj, Instance>: Debug + DynClone {
    // Produce something that can be used to identify types.
    fn id(&self) -> &TypeId;
    fn instance(&self) -> Instance;
    fn to_obj(self) -> Obj;
}

/// Derived from a type. 
/// 
/// Some type instances are simple and thus are just static-- aka they can immutable.
/// Other type instances will be more complex, having its own data and some parts that are static, a nested trait.
/// 
/// Nonetheless, they both are behind the same `TypeInstance<Obj>` trait
pub trait Instance<Obj>: Debug + DynClone{
    fn type_id_name<'a>(&'a self) -> &'a String{
        self.type_id().string()
    }
    fn type_id(&self) -> &TypeId;
    fn data_to_string(&self) -> Option<String>;
    fn load_string(&mut self, data: String);
}

#[cfg(test)]
mod tests{
    

    //TODO: test type list ptr
}