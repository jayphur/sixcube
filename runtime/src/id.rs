use prelude::*;
use core_obj::ObjStruct;

#[derive(Debug, Clone, Copy)]
pub(crate) struct TypeId<'i>(&'i TypeIdInner);
impl<'i> core_obj::TypeId for TypeId<'i>{
    type AttrId = AttrId<'i>;
    fn my_obj(&self) -> &ObjStruct {
        &self.0.object
    }
}
impl PartialEq for TypeId<'_>{
    fn eq(&self, other: &Self) -> bool {
        //FIXME: is this reliable?? i know its fast... but reliable?
        (self.0 as *const TypeIdInner) as u32 ==  (other.0 as *const TypeIdInner) as u32
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypeIdInner{
    object: ObjStruct,
    replace_this: u128,
}

#[derive(Debug,Clone, Copy, PartialEq)]
pub struct AttrId<'i>(&'i AttrIdInner);
impl<'i> core_obj::AttrId for AttrId<'i>{
    fn default_inner(&self) -> core_obj::Attr<Self> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AttrIdInner{

}