use std::fmt::Debug;
use core_obj::Pos;
use prelude::*;

use crate::{Pos16, PosU};


#[derive(Debug, Clone)]
pub struct BgTree<T: Debug + Clone +Copy>{
    vec: T,
}
impl<T: Debug + Clone +Copy,> BgTree<T>{
    pub fn pop_and_finish<const SIZE: usize,F: Fn(Pos) -> T>(&mut self, cw_pos: Pos16, get: F) 
        -> Result<[[[T;SIZE];SIZE];SIZE]>{
        todo!()
    }
    //TODO: Once i know how drawing these outer chunks will work, i can finish how this thing does it's stuff...
} 

//oct tree where each node holds a value

enum Node<T: Debug + Clone +Copy>{
    Single{
        t: T
    },
    Parent{
        t: T,
        children: [Box<Node<T>>;7]
    }
}

impl<T: Debug + Clone + Copy> Node<T> {
    fn get(&self, pos: PosU) -> Option<(T,Size)>{
        todo!()
    }
    fn ensure<F: Fn(Pos) -> T>(&mut self, pos: PosU, f: F){
        todo!()
    }
    fn ensure_all<F: Fn(Pos) -> T>(&mut self, f: F){
        todo!()
    }
    fn become_parent<F: Fn(Pos) -> T>(self, f: F) -> Node<T>{
        todo!()
    }
}


#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size(usize);

impl Size {
    pub fn get(&self) -> u32{
        2^self.0 as u32
    }
}
