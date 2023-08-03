use core::panic;
use std::ops::Deref;

use sc_prelude::*;

use super::GrowingOctree;

type Pos = (i16,i16,i16);

#[derive(Debug)]
pub enum Octree<T: Default + Debug>{
    LeafNode(LeafInner<T>),
    Node{
        n: NodeInner<Box<Self>>, 
        size: u16
    }, 
}
impl<T: Default + Debug> Octree<T>{
    fn new(t: T) -> Self{
        todo!()
    }
    /// non-root
    fn new_nodes(t: T, at: Pos, size: u16) -> Self{
        let size = size as i16;
        if at.0 > 0 || at.1 > 0 || at.2 > 0 { 
            panic!("non-root nodes (themselves, without context of the root) represent a negative position!") }
        if at.0 > size || at.0 > size || at.0 > size { 
            panic!("attempting to create nodes with a position outside of bounds (as defined by size: u16)!")}
        if at == (0,0,0){
            Self::LeafNode(LeafInner(t))
        }
        else{
            todo!() //nodes
        }
    }

}

impl<T: Default + Debug> Default for Octree<T>{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Default + Debug> GrowingOctree<T> for Octree<T>{
    fn get_weak(&self, pos: Pos) -> Option<&T> {
        match self{
            Octree::LeafNode(leaf) => {
                leaf.get(pos)
            },
            Octree::Node{n, size} => {
                n.get(pos, *size)?.get_weak(pos)
            },
        }
    }

    fn get_mut_weak(&mut self, pos: Pos) -> Option<&mut T> {
        match self{
            Octree::LeafNode(leaf) => {
                leaf.get_mut(pos)
            },
            Octree::Node{n, size} => {
                n.get_mut(pos, *size)?.as_deref_mut()?.get_mut_weak(pos)
            },
        }
    }

    fn get_mut_strong(&mut self, pos: Pos) -> &mut T {
        match self{
            Octree::LeafNode(leaf) => {
                leaf.get_mut(pos).expect(
            "Attempting to access a leaf node with a non-origin position. 
                This should not happen in a get_mut_strong call. 
                The above nodes should guarantee that this does not happen by creating necessary nodes.")
            },
            Octree::Node{n, size} => {
                let Some(t) = n.get_mut(pos, *size) else {
                    // we must create nodes and crap.
                    todo!()
                };
                let Some(t) = t else {
                    // The position is housed in the tree, but is not created.
                    if *size == 1 {
                        let leaf = LeafInner(T::default());
                        *t = Some(Box::new(Octree::LeafNode(leaf)));
                        &mut leaf.0

                    } else {

                    }
                };
                t.get_mut_strong(pos)
            },
        }
    }
}

#[derive(Debug)]
pub struct LeafInner<T>(T);

impl<T> LeafInner<T>{
    fn get(&self, pos: Pos) -> Option<&T>{
        if pos == (0,0,0){
            return Some(&self.0);
        } else {
            return None;
        }
    }
    fn get_mut(&mut self, pos: Pos) -> Option<&mut T>{
        if pos == (0,0,0){
            return Some(&mut self.0);
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
pub struct NodeInner<T>{
    // each node's child is 0 to 1 units displaced in each axis from (0,0,0).
    children: [[[Option<T>;2]; 2]; 2], //starting in top back left corner.

    // if this is true, we know that this is the first node in the tree. (the negative nodes are shifted - 1)
    root: bool,

    // EX: the root is child_size = 4. This children are from -4 to 0
    // these children's children are now 0 to 2.
    // THOSE children's children are now 0 to 1.
    // thus, the root's child_size is the range as such: [-size, size)
    // the "ranges" of the non-root nodes are the min & max displacement the node can do: [0, 2*size) 
}
impl<T> NodeInner<T>{
    fn resize_pos(&self, pos: Pos, size: i16) -> Pos{
        (pos.0 % size, pos.1 % size, pos.2 % size)
    }
    /// assuming pos as been sized...
    fn pos_fits(&self, pos: &Pos, size: i16) -> bool{
        if self.root{
            pos.0 < size && pos.0 >= -size &&
            pos.1 < size && pos.1 >= -size &&
            pos.2 < size && pos.2 >= -size
        } else {
            true
        }
    }
    fn get_mut(&mut self, mut pos: Pos, size: u16) -> Option<&mut Option<T>>{
        pos = self.resize_pos(pos, size as i16);
        if !self.pos_fits(&pos, size as i16) {return None;}
        else{
            Some(self.pos_to_child_mut(&pos))
        }
    }
    fn get(&self, mut pos: Pos, size: u16) -> Option<&T>{
        pos = self.resize_pos(pos, size as i16);
        if !self.pos_fits(&pos, size as i16) {return None;}
        else{
            self.pos_to_child(&pos).as_ref()
        }
    }
    fn pos_to_child(&self, pos: &Pos) -> &Option<T>
    {
        let f = |val: &i16| {
            if self.root{
                val.is_positive() as usize
            } else {
                if *val > 0 { panic!("we should not get here because we have bounds checking with pos_fits(...) -> bool"); }
                val.is_negative() as usize
            }
        };

        &self.children[f(&pos.0)][f(&pos.1)][f(&pos.2)]
    }
    fn pos_to_child_mut(&mut self, pos: &Pos) -> &mut Option<T>
    {
        let f = |val: &i16| {
            if self.root{
                val.is_positive() as usize
            } else {
                if *val > 0 { panic!("we should not get here because we have bounds checking with pos_fits(...) -> bool"); }
                val.is_negative() as usize
            }
        };
        &mut self.children[f(&pos.0)][f(&pos.1)][f(&pos.2)]
    }
}
