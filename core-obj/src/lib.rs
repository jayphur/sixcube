use std::ops::Add;

use serde::{Deserialize, Serialize};

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

    /// DataContainers:
    ///
    /// How the data is formatted, aka the containers composition: Responsibility of the Runtime
    ///
    /// How the data is used: Responsibility of the plugin.
    type DataContainer: Sync + Send + Clone + Default + Debug + for <'a> Deserialize<'a> + PartialEq;
}

/// A type, as part of the user defined plugin system
pub trait RuntimeType{
    fn name(&self) -> &String;
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Value{
    Bool(bool),
    U16(u16),
    I16(i16),
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
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
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct LocalPos(pub i8,pub i8,pub i8);
impl LocalPos{
    #[inline]
    pub fn from_usize(from: (usize,usize,usize)) -> Self{
        Self(from.0 as i8,from.1 as i8,from.2 as i8)
    }
    
}

/// Data that a voxel, or dim, might store based on its functionality. (ie an inventory, values such as heat, energy, etc)
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Data{
    Inventory(Vec<()>), //TODO: Items instead of "()"
    Pos(Pos),
    LocalPos(LocalPos),
    Value(i32),
    List(Vec<Data>),
    #[default]
    None,
}
