use core::panic;
use std::ops::Deref;

use sc_core::obj::pos::LocalPos;
use sc_prelude::*;

use super::GrowingOctree;

type Pos = (i16,i16,i16);
type PosLocal = (u16,u16,u16);

#[derive(Debug)]
pub struct Octree<T: Default + Debug>{
    root: RootNode<T>,
}
impl<T: Default + Debug> Octree<T>{
    fn new(t: T) -> Self{ //t: T might be removed
        todo!()
    }
}

impl<T: Default + Debug> Default for Octree<T>{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Default + Debug> GrowingOctree<T> for Octree<T>{
    fn get_weak(&self, pos: (i16,i16,i16)) -> Option<&T> {
        self.root.get_weak(pos)
    }

    fn get_mut_weak(&mut self, pos: (i16,i16,i16)) -> Option<&mut T> {
        self.root.get_mut_weak(pos)
    }

    fn get_mut_strong(&mut self, pos: (i16,i16,i16)) -> &mut T {
        self.root.get_mut_strong(pos)
    }
}

#[derive(Debug)]
enum NodeChild<T>{
    Empty,
    Leaf(T),
    Node(Box<BranchNode<T>>),
}
// each node's child is 0 to 1 units displaced in each axis from (0,0,0).

// EX: the root is child_size = 4. This children are from -4 to 0
// these children's children are now 0 to 2.
// THOSE children's children are now 0 to 1.
// thus, the root's child_size is the range as such: [-size, size)
// the "ranges" of the non-root nodes are the min & max displacement the node can do: [0, 2*size) 

#[derive(Debug)]
struct RootNode<T>{
    children: [[[NodeChild<T>;2]; 2]; 2], //starting in top back left corner.
    size: u16,
}
impl<T: Default + Debug> GrowingOctree<T> for RootNode<T>{
    fn get_weak(&self, pos: (i16,i16,i16)) -> Option<&T> {
        todo!()
    }

    fn get_mut_weak(&mut self, pos: (i16,i16,i16)) -> Option<&mut T> {
        todo!()
    }

    fn get_mut_strong(&mut self, pos: (i16,i16,i16)) -> &mut T {
        todo!()
    }
}
impl<T: Debug + Default> Default for RootNode<T>{
    fn default() -> Self {
        todo!()
    }
}
impl<T: Debug + Default> RootNode<T>{
    /// Convert this global position to one local to any of my children (usless without global_pos_to_child_index())
    fn global_pos_to_local(&self, global: Pos) -> Option<LocalPos>{
        None
    }
    /// Which child will store this global position, if any.
    fn global_pos_to_child_index(&self, global: Pos) -> Option<LocalPos>{
        None
    }
}
#[derive(Debug)]
struct BranchNode<T>{
    children: [[[NodeChild<T>;2]; 2]; 2], //starting in top back left corner.
    // there is no "size: u16" here.
    // branch nodes don't actually store their "size". 
    // Instead, it is found as you progress down from the root
}
impl<T> BranchNode<T>{

}

/// Converts, panicing if any of the values in the "global" Pos are negative
fn unwrap_global_to_local_pos(pos: Pos) -> LocalPos{
    todo!()
}