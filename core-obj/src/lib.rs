use std::{marker::PhantomData, ops::Add};
use serde::{Serialize, Deserialize};
use prelude::*;

pub trait Runtime: Sized + Debug + Clone + Send + Sync{

    type VoxelType: RuntimeType + Send + Sync + Debug + Clone + Copy + Serialize + for<'a> Deserialize<'a> + PartialEq;
    fn get_voxels(&self) -> &[Self::VoxelType];
    fn voxel_default(&self, r#type: &Self::VoxelType) -> (); //Something
    fn find_voxel_by_name(&self, name: String) -> &Self::VoxelType;

    type AttrType: RuntimeType + Send + Sync + Debug + Clone + Copy + Serialize + for <'a> Deserialize<'a> + PartialEq;
    fn get_attr(&self) -> &[Self::AttrType];
    fn attr_default(&self, r#type: &Self::AttrType) -> Value;
    fn find_attr_by_name(&self, name: String) -> &Self::AttrType;

}
pub trait RuntimeType{
    fn name(&self) -> &String;
}

pub trait Voxel<R: Runtime>{
    fn read_type(&self) -> &R::VoxelType;
    fn read_data(&self) -> (); //TODO: finish this signature
    fn write_type(&mut self, val: R::VoxelType);
    fn write_data(&mut self); //TODO: finish this signature
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

impl Add<Self> for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Pos{
    #[inline]
    pub fn from_usize(from: (usize,usize,usize)) -> Self{
        Self(from.0 as i32,from.1 as i32,from.2 as i32)
    }
    
}


// maybe delete
#[derive(Debug, Default, Clone, Copy)]
pub struct LocalPos(pub i8,pub i8,pub i8);
impl LocalPos{
    #[inline]
    pub fn from_usize(from: (usize,usize,usize)) -> Self{
        Self(from.0 as i8,from.1 as i8,from.2 as i8)
    }
    
}
