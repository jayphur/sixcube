use std::{marker::PhantomData, ops::Add};
use serde::{Serialize, Deserialize};
use prelude::*;

pub trait Runtime: Sized + Debug + Clone + Send + Sync{
    type VoxelType: Send + Sync + Debug + Clone + Copy + Serialize + for<'a> Deserialize<'a> + PartialEq;
    fn get_voxels(&self) -> &[Self::VoxelType];
    fn voxel_default(&self, r#type: &Self::VoxelType) -> &Voxel<Self>;
    fn voxel_info(&self, r#type: &Self::VoxelType) -> &TypeInfo;

    type AttrType: Send + Sync + Debug + Clone + Copy + Serialize + for <'a> Deserialize<'a> + PartialEq;
    fn get_attr(&self) -> &[Self::AttrType];
    fn attr_default(&self, r#type: &Self::AttrType) -> &Attr<Self>;
    fn attr_info(&self, r#type: &Self::AttrType) -> &TypeInfo;
}

#[derive(Debug)]
pub struct TypeInfo{
    pub name: String,
    pub plugin: String,
}

#[derive(Debug)]
pub struct Voxel<R: Runtime>{
    pub my_type: R::VoxelType
}
impl<R: Runtime> Clone for Voxel<R>{
    fn clone(&self) -> Self {
        Self { my_type: self.my_type.clone() }
    }
}


#[derive(Debug)]
pub struct Attr<R: Runtime>{
    pub my_type: R::AttrType,
}
impl<R: Runtime> Clone for Attr<R>{
    fn clone(&self) -> Self {
        Self { my_type: self.my_type.clone() }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Value{
    Bool(bool),
    U16(u16),
    I16(i16),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Pos(pub i32,pub i32,pub i32);
impl Pos{
    #[inline]
    pub fn from_usize(from: (usize,usize,usize)) -> Self{
        Self(from.0 as i32,from.1 as i32,from.2 as i32)
    }
}
