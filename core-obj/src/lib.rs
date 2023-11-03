use std::{fmt::Debug, marker::PhantomData};

pub type Pos = vector3d::Vector3d<i32>;

pub trait Voxel: Debug + Clone{
    type Type: Type<Self::AttrType, Obj = Self, >;
    type AttrType: AttrType;
    type Data;

    fn get_type(&self) -> &Self::Type;
}

pub trait Dim<V: Voxel>: Debug + Clone{
    type TypeId: Type<Self::AttrType, Obj = Self>; 
    type AttrType: AttrType;
    type Data;
}

#[derive(Debug, Default, Clone)]
pub struct World<V: Voxel,D: Dim<V>> {
    pub dims:         Vec<D>,
    __marker: PhantomData<V>
}

/// Assuming there is no mixing type id of different objects by faith (and checking)
pub trait Type<A: AttrType>: PartialEq + Copy + Clone + Debug + Send + Sync {
    type Obj;
}

pub trait Action: PartialEq + Copy + Clone + Send + Sync {
    
}

pub trait AttrType: PartialEq + Copy + Clone + Debug + Send + Sync{
    type Obj;
    fn new(&self) -> Attr<Self>;
}

#[derive(Debug, Default, Clone)]
pub struct Attr<A: AttrType> {
    pub my_type: A,
    pub val: AttrValue,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum AttrValue {
    #[default]
    Unset,
    Boolean(bool),
    U8(u8),
    String(String),
}


//TODO: why is this here, remove this??
/// An object that can store some data
pub trait Data: Default + Debug + Clone + Send + Sync {
    type Property: Copy + Clone;
    type Value;
    fn get(&self, prop: Self::Property) -> Option<&Self::Value>;
    fn get_mut(&mut self, prop: Self::Property) -> Option<&mut Self::Value>;
    fn set(&mut self, prop: Self::Property, val: Self::Value);
}
