use std::ops::Add;

use serde::{Deserialize, Serialize};

use prelude::*;
pub use registrar::*;

pub mod fake;
mod registrar;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Value{
    Bool(bool),
    U16(u16),
    I16(i16),
}


#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct PosU(pub usize,pub usize,pub usize);

impl PosU {
    pub fn tuple(&self) -> (usize,usize,usize){
        (self.0,self.1,self.2)
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalPos(pub i8,pub i8,pub i8);
impl LocalPos{
    #[inline]
    pub fn from_usize(from: (usize,usize,usize)) -> Self{
        Self(from.0 as i8,from.1 as i8,from.2 as i8)
    }
    
}

/// Data that a voxel, or dim, might store based on its functionality. (ie an inventory, values such as heat, energy, etc.)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Data{
    Inventory(Vec<()>), //TODO: Items instead of "()"
    Pos(Pos),
    LocalPos(LocalPos),
    Value(i32),
    List(Vec<Data>),
    #[default]
    None,
}
