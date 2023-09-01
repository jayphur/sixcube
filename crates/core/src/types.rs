use dyn_clone::DynClone;
use rustc_hash::FxHashMap;
use std::fmt::Debug;
use crate::{data::Name, obj::voxel::VoxelTypePtr};

/// Type of something. a block / item / etc. Can be applied to "obj"s.
/// This is the SINGLE static "master" variant of this trait.
/// Most likely the `TypeInstance`s will have some kind of pointer back to this master 
/// `Type`.
pub trait Type<Obj, Instance>: Debug + DynClone {
    fn name(&self) -> &Name;
    fn instance(&self) -> Instance;
}

/// Derived from a type. 
/// 
/// Some type instances are simple and thus are just static-- aka they can immutable.
/// Other type instances will be more complex, having its own data and some parts that are static, a nested trait.
/// 
/// Nonetheless, they both are behind the same `TypeInstance<Obj>` trait
pub trait TypeInstance<Obj>: Debug + DynClone {
    fn name(&self) -> &Name;
    fn to_obj(self) -> Obj;
}

#[derive(Debug, Default)]
pub struct TypeListPtr{
    voxel: FxHashMap<Name, VoxelTypePtr>,
}
impl TypeListPtr{
    fn get_voxel(&self, name: &Name) -> Option<&VoxelTypePtr>{
        self.voxel.get(name)
    }
    fn add_voxel(&mut self, ptr: VoxelTypePtr){
        self.voxel.insert(ptr.as_ref().name().clone(), ptr);
    }
}


#[cfg(test)]
mod tests{
    

    //TODO: test type list ptr
}