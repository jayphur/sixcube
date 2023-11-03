use core_obj::{AttrType, AttrValue};
use world_db::Map;

use crate::{voxel::Voxel, data::Data};


#[derive(Debug, Clone)]
pub struct Dim<'i>{
    my_type: &'i DimType,
    map: Map<Voxel<'i>>,
}
impl<'i> core_obj::Dim<Voxel<'i>> for Dim<'i>{
    type TypeId = &'i DimType;
    type AttrType = &'i DimAttr;
    type Data = Data;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DimType{
    uuid: u32,
}
impl PartialEq for DimType{
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
impl<'i> core_obj::Type<&'i DimAttr> for &'i DimType{
    type Obj = Dim<'i>;
}


#[derive(Debug)]
pub struct DimAttr{
    uuid: u32,
    default_value: AttrValue,
}
impl PartialEq for DimAttr{
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
impl<'i> AttrType for &'i DimAttr{
    type Obj = Dim<'i>;

    fn new(&self) -> core_obj::Attr<Self> {
        core_obj::Attr{
            my_type: self.clone(), // It is my intention to clone the reference.
            val: self.default_value.clone(),
        }
    }
}