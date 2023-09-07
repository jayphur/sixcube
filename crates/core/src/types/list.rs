use rustc_hash::FxHashMap;

use crate::{TypeId, obj::voxel::VoxelTypePtr};

#[derive(Debug, Default)]
pub struct TypePtrList{
    voxel: FxHashMap<TypeId, VoxelTypePtr>,
}
impl TypePtrList{
    pub fn get_voxel<T: PartialEq<TypeId>>(&self, name: T) -> Option<&VoxelTypePtr>{
        self.voxel.iter()
            .find_map(|(id, ptr)| 
                if name == *id { Some(ptr)} 
                else {None})
    }
    pub fn add_voxel(&mut self, ptr: VoxelTypePtr){
        self.voxel.insert(ptr.as_ref().id().clone(), ptr);
    }
}
