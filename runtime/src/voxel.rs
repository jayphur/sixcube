use core_obj::{AttrType, Value, ActionType};
use world_db::Map;

use crate::{data::Data};


#[derive(Debug, Clone)]
pub struct Voxel<'i>{
    my_type: &'i VoxelType,
}
impl<'i> core_obj::Voxel for Voxel<'i>{
    type Type = &'i VoxelType;
    type AttrType = &'i VoxelAttr;
    type ActionType = &'i VoxelAction;
    type Data = Data;

    fn get_type(&self) -> &Self::Type {
        &self.my_type
    }

}

#[derive(Debug, Default, Clone, Copy)]
pub struct VoxelType{
    uuid: u32,
    // pub fields: ...
}
impl PartialEq for VoxelType{
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
impl<'i> core_obj::Type<&'i VoxelAttr> for &'i VoxelType{
    type Obj = Voxel<'i>;
}


#[derive(Debug)]
pub struct VoxelAttr{
    uuid: u32,
    default_value: Value,
}
impl PartialEq for VoxelAttr{
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
impl<'i> AttrType for &'i VoxelAttr{
    type Obj = Voxel<'i>;

    fn new(&self) -> core_obj::Attr<Self> {
        core_obj::Attr{
            my_type: self.clone(), // It is my intention to clone the reference.
            val: self.default_value.clone(),
        }
    }
}

#[derive(Debug)]
pub struct VoxelAction{
    uuid: u32,  
}
impl PartialEq for VoxelAction{
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
impl<'i> ActionType for &'i VoxelAction{

}