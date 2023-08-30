use dyn_clone::DynClone;
use rustc_hash::FxHashMap;
use std::fmt::Debug;
use crate::{data::Name, obj::{voxel::VoxelTypePtr, element::ElementTypePtr, dim::DimTypePtr}};

/// Type of something. a block / item / etc. Can be applied to "obj"s.
/// A SINGLE static "master" variant of this trait (should) exist for all dyn Type<Obj>s of a specific variant.
/// It can be accessed exclusively behind a 'static reference, or not...
pub trait ObjType<Obj>: Debug + DynClone {
    fn name(&self) -> &Name;
    fn is_master(&self) -> bool;
    fn new_obj(&self) -> Obj;
}

#[derive(Debug, Default)]
pub struct TypeListPtr{
    voxel: FxHashMap<Name, VoxelTypePtr>,
    element: FxHashMap<Name, ElementTypePtr>,
    dim: FxHashMap<Name, DimTypePtr>
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