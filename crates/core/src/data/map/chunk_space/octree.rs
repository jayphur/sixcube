use core::panic;
use sc_prelude::*;

use super::GrowingOctree;

type GlobalPos = (i16,i16,i16);
type PosUnsigned = (u16,u16,u16);

#[derive(Debug)]
pub struct Octree<T: Default + Debug>{
    root: RootNode<T>,
}
impl<T: Default + Debug> Octree<T>{
    ///size, what the largest - and + size for the tree should be at creation
    fn new(size: u16) -> Self{ //t: T might be removed
        Self { 
            root: RootNode { children: Default::default(), size } 
        }
    }
}

impl<T: Default + Debug> Default for Octree<T>{
    fn default() -> Self {
        Self::new(4)
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

    fn new(must_include: i16) -> Self {
        let size = (must_include.abs() as usize).next_power_of_two() as u16;
        Self::new(size)
    }
}

#[derive(Default, Debug)]
enum NodeChild<T: Debug + Default>{
    #[default]
    Empty,
    Leaf(T),
    Node(Box<BranchNode<T>>),
}
impl<T: Debug + Default> NodeChild<T>{
    fn get_weak(&self, pos: (u16,u16,u16), size: u16) -> Option<&T> {
        match self{
            NodeChild::Empty => None,
            NodeChild::Leaf(t) => {
                if pos != (0,0,0) || size != 0 { panic!("tree structure is malformed/unbalanced") }
                Some(t)
            },
            NodeChild::Node(node) => {
                node.get_weak(pos, size)
            },
        }
    }

    fn get_mut_weak(&mut self, pos: (u16,u16,u16), size: u16) -> Option<&mut T> {
        match self{
            NodeChild::Empty => None,
            NodeChild::Leaf(t) => {
                if pos != (0,0,0) || size != 0 { panic!("tree structure is malformed/unbalanced") }
                Some(t)
            },
            NodeChild::Node(node) => {
                node.get_mut_weak(pos, size)
            },
        }
    }

    fn get_mut_strong(&mut self, pos: (u16,u16,u16), size: u16) -> &mut T {
        if pos == (0,0,0){
            match self{
                NodeChild::Empty => {
                    *self = NodeChild::Leaf(T::default());
                    match self{ 
                        NodeChild::Leaf(t) => t, 
                        _ => unreachable!()
                    }
                },
                NodeChild::Leaf(leaf) => leaf,
                _ => panic!("tree structure is malformed/unbalanced"),
            }
        } else {
            match self{
                NodeChild::Empty => {
                    *self = NodeChild::Node(Box::new(BranchNode::new()));
                    match self{ 
                        NodeChild::Node(node) => node.get_mut_strong(pos, size), 
                        _ => unreachable!()
                    }
                },
                NodeChild::Node(node) => node.get_mut_strong(pos, size),
                _ => panic!("tree structure is malformed/unbalanced"),
            }
        }
    }
}
// each node's child is 0 to 1 units displaced in each axis from (0,0,0).

// EX: the root is child_size = 4. This children are from -4 to 0
// these children's children are now 0 to 2.
// THOSE children's children are now 0 to 1.
// thus, the root's child_size is the range as such: [-size, size)
// the "ranges" of the non-root nodes are the min & max displacement the node can do: [0, 2*size) 

#[derive(Debug)]
struct RootNode<T: Debug + Default>{
    children: [[[NodeChild<T>;2]; 2]; 2], //starting in top back left corner.
    size: u16,
}
impl<T: Default + Debug> RootNode<T>{
    fn get_weak(&self, pos: (i16,i16,i16)) -> Option<&T> {
        let pos = self.global_pos_to_local(pos)?;
        self.children
            [pos.child.2 as usize]
            [pos.child.1 as usize]
            [pos.child.0 as usize]
            .get_weak(pos.position_in_child, self.size / 2)
    }

    fn get_mut_weak(&mut self, pos: (i16,i16,i16)) -> Option<&mut T> {
        let pos = self.global_pos_to_local(pos)?;
        self.children
            [pos.child.2 as usize]
            [pos.child.1 as usize]
            [pos.child.0 as usize]
            .get_mut_weak(pos.position_in_child, self.size / 2)
    }

    fn get_mut_strong(&mut self, pos: (i16,i16,i16)) -> &mut T {
        let Some(pos) = self.global_pos_to_local(pos) else {
            todo!() //expand 
        };
        self.children
            [pos.child.2 as usize]
            [pos.child.1 as usize]
            [pos.child.0 as usize]
            .get_mut_strong(pos.position_in_child, self.size / 2)
    }
}
/// A position relative to a parent node that exists in one of its children.
struct NodeLocalPos{
    child: (bool, bool, bool),
    position_in_child: PosUnsigned,
}
impl<T: Debug + Default> RootNode<T>{
    /// Convert this global position to a local one with a corresponding child (if possible ofc)
    fn global_pos_to_local(&self, global: GlobalPos) -> Option<NodeLocalPos>{
        let size = self.size as i16;
        let is_in_bounds = |val: i16| { -size <= val && size > val };
        if !is_in_bounds(global.0) || !is_in_bounds(global.1) || !is_in_bounds(global.2){
            return None;
        }
        let convert = |val: i16| {
            let index = val.is_positive();
            if index {
                (index, (val - size) as u16)
            } else {
                (index, (val + size) as u16)
            }
        };
        let converted = (convert(global.0),convert(global.1),convert(global.2));
        Some(NodeLocalPos{
            child: (converted.0.0,converted.1.0,converted.2.0),
            position_in_child: (converted.0.1,converted.1.1,converted.2.1),
        })
    }
}
#[derive(Debug)]
struct BranchNode<T: Debug + Default>{
    children: [[[NodeChild<T>;2]; 2]; 2], //starting in top back left corner.
    // there is no "size: u16" here.
    // branch nodes don't actually store their "size". 
    // Instead, it is found as you progress down from the root
}
impl<T: Debug + Default> BranchNode<T>{
    fn new() -> Self{
        Self { children: Default::default() }
    }
    fn get_weak(&self, pos: PosUnsigned, size: u16) -> Option<&T>{
        let (index, pos) = Self::reduce_pos(pos,size);
        self.children[index.2 as usize][index.1 as usize][index.0 as usize].get_weak(pos, size / 2)
    }
    fn get_mut_weak(&mut self, pos: PosUnsigned, size: u16) -> Option<&mut T>{
        let (index, pos) = Self::reduce_pos(pos,size);
        self.children[index.2 as usize][index.1 as usize][index.0 as usize].get_mut_weak(pos, size / 2)
    }
    fn get_mut_strong(&mut self, pos: PosUnsigned, size: u16) -> &mut T{
        let (index, pos) = Self::reduce_pos(pos,size);
        self.children[index.2 as usize][index.1 as usize][index.0 as usize].get_mut_strong(pos, size / 2)
    }
    fn reduce_pos(pos: PosUnsigned, size: u16) -> ((usize,usize,usize), PosUnsigned){
        let index = |val: u16| { val < size };
        let index = 
            (index(pos.0),
            index(pos.1),
            index(pos.2));
        let pos = 
            ((pos.0 - index.0 as u16 * size),
            (pos.0 - index.0 as u16 * size),
            (pos.0 - index.0 as u16 * size));
        ((index.0 as usize, index.1 as usize, index.2 as usize), pos)
    }
}