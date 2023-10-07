use core::panic;
use prelude::*;
use core_obj::Pos;

use super::OctreeTrait;

type GlobalPos = (i32, i32, i32);
type PosUnsigned = (u16, u16, u16);

#[derive(Debug)]
pub struct Octree<T: Default + Debug> {
    root: RootNode<usize>,
    memory: Vec<T>,
}
impl<T: Default + Debug> Octree<T> {
    ///size, what the largest - and + size for the tree should be at creation
    fn new(size: u16) -> Self {
        //t: T might be removed
        Self {
            root: RootNode {
                children: Default::default(),
                size: (size as usize + 1).next_power_of_two() as u16,
            },
            memory: Vec::with_capacity((size as usize + 1).next_power_of_two()),
        }
    }
}

impl<T: Default + Debug + Send> Default for Octree<T> {
    fn default() -> Self {
        Self::new(4)
    }
}

impl<T: Default + Debug + Send> OctreeTrait<T> for Octree<T> {
    fn get_weak(&self, pos: &Pos ) -> Option<&T> {
        self.get_raw(*self.root.get_weak((pos.x,pos.y,pos.z))?)
    }

    fn get_mut_weak(&mut self, pos: &Pos) -> Option<&mut T> {
        self.memory.get_mut(*self.root.get_mut_weak((pos.x,pos.y,pos.z))?)
    }

    fn get_mut_strong(&mut self, pos: &Pos ) -> &mut T {
        let index = *self.root.get_mut_strong((pos.x,pos.y,pos.z), self.memory.len());
        if index == self.memory.len(){
            self.memory.push(T::default());
        }
        self.get_raw_mut(index).unwrap()
    }

    fn new(size: u16) -> Self {
        let size = (size as usize).next_power_of_two() as u16;
        Self::new(size)
    }

    fn find_index(&self, pos: &Pos) -> Option<usize> {
        self.root.get_weak((pos.x,pos.y,pos.z)).copied()
    }

    fn get_raw(&self, index: usize) -> Option<&T> {
        self.memory.get(index)
    }

    fn get_raw_mut(&mut self, index: usize) -> Option<&mut T> {
        self.memory.get_mut(index)
    }

    fn slice_raw(&self) -> &[T] {
        self.memory.as_slice()
    }

    fn slice_raw_mut(&mut self) -> &mut [T] {
        self.memory.as_mut_slice()
    }

    fn get_raw_many_mut(&mut self, many: &Vec<usize>) -> Vec<&mut T> {
        let mut current: usize = 0;
        let mut mut_ref = self.memory.iter_mut();
        let mut output = Vec::with_capacity(many.len());
        for index in many{
            current = index - current;
            output.push(mut_ref.nth(current).unwrap())
        }
        output
    }

}

#[derive(Default, Debug)]
enum NodeChild<T: Debug + Default> {
    #[default]
    Empty,
    Leaf(T),
    Node(Box<BranchNode<T>>),
}
impl<T: Debug + Default> NodeChild<T> {
    fn get_weak(&self, pos: (u16, u16, u16), size: u16) -> Option<&T> {
        match self {
            NodeChild::Empty => None,
            NodeChild::Leaf(t) => {
                if pos != (0, 0, 0) || size > 1 {
                    panic!("tree structure is malformed/unbalanced")
                }
                Some(t)
            }
            NodeChild::Node(node) => node.get_weak(pos, size),
        }
    }

    fn get_mut_weak(&mut self, pos: (u16, u16, u16), size: u16) -> Option<&mut T> {
        match self {
            NodeChild::Empty => None,
            NodeChild::Leaf(t) => {
                if pos != (0, 0, 0) || size > 1 {
                    panic!("tree structure is malformed/unbalanced")
                }
                Some(t)
            }
            NodeChild::Node(node) => node.get_mut_weak(pos, size),
        }
    }

    fn get_mut_strong(&mut self, pos: (u16, u16, u16), size: u16, new_t: T) -> &mut T {
        if size == 0 {
            match self {
                NodeChild::Empty => {
                    *self = NodeChild::Leaf(new_t);
                    match self {
                        NodeChild::Leaf(t) => t,
                        _ => unreachable!(),
                    }
                }
                NodeChild::Leaf(leaf) => leaf,
                _ => panic!("tree structure is malformed/unbalanced"),
            }
        } else {
            match self {
                NodeChild::Empty => {
                    *self = NodeChild::Node(Box::new(BranchNode::new()));
                    self.get_mut_strong(pos, size, new_t)
                }
                NodeChild::Node(node) => node.get_mut_strong(pos, size, new_t),
                _ => panic!("tree structure is malformed/unbalanced"),
            }
        }
    }
}
// each node's child is 0 to 1 units displaced in each axis from (0,0,0).

// EX: the root is child_size = 4. This children are -4 and 0
// those children's children are now 0 to 2.
// THOSE children's children are now 0 to 1.
//
// the children on the "left side" (the ones that start with -4) are [-4,-1]
// (-1 because the largest number they can have is 3. -4 + 3 = -1)
//
// the children on the "right" (starting from zero) are just [0,3]
//
// thus, the root's child_size is the range as such: [-size, size)
// the "ranges" of the non-root nodes are the min & max displacement the node can do: [0, 2*size)

#[derive(Debug)]
struct RootNode<T: Debug + Default> {
    children: [[[NodeChild<T>; 2]; 2]; 2], //starting in top back left corner.
    size: u16,                             // the width of the level is 2*size
}
impl<T: Default + Debug> RootNode<T> {
    fn get_weak(&self, pos: (i32, i32, i32)) -> Option<&T> {
        let pos = self.global_pos_to_local(pos)?;
        self.children[pos.child.2 as usize][pos.child.1 as usize][pos.child.0 as usize]
            .get_weak(pos.position_in_child, self.size / 2)
    }

    fn get_mut_weak(&mut self, pos: (i32, i32, i32)) -> Option<&mut T> {
        let pos = self.global_pos_to_local(pos)?;
        self.children[pos.child.2 as usize][pos.child.1 as usize][pos.child.0 as usize]
            .get_mut_weak(pos.position_in_child, self.size / 2)
    }

    fn get_mut_strong(&mut self, pos: (i32, i32, i32), new_t: T) -> &mut T {
        let Some(pos) = self.global_pos_to_local(pos) else {
            // growing.
            // increase - to *2 and + to *2, then the - must be +2 within the -2 child 
            // and the + must be 0 within the +2 child
            // thus no net change.
            let mut new = RootNode::<T>{
                children: Default::default(),
                size: self.size*2,
            };
            for x in 0usize..2usize{
                for y in 0usize..2usize{
                    for z in 0usize..2usize{
                        let mut node = BranchNode::default();
                        node.children[z ^ 1][y ^ 1][x ^ 1] = std::mem::take(&mut self.children[z][y][x]);
                        new.children[z][y][x] = NodeChild::Node(Box::new(node))
                    }
                }
            }
            *self = new;
            return self.get_mut_strong(pos, new_t);
        };
        self.children[pos.child.2 as usize][pos.child.1 as usize][pos.child.0 as usize]
            .get_mut_strong(pos.position_in_child, self.size / 2, new_t)
    }
}
/// A position relative to a parent node that exists in one of its children.
struct NodeLocalPos {
    child: (bool, bool, bool),
    position_in_child: PosUnsigned,
}
impl<T: Debug + Default> RootNode<T> {
    /// Convert this global position to a local one with a corresponding child (if possible ofc)
    fn global_pos_to_local(&self, global: GlobalPos) -> Option<NodeLocalPos> {
        let size = self.size as i32;
        let is_in_bounds = |val: i32| -size <= val && size > val;
        if !is_in_bounds(global.0) || !is_in_bounds(global.1) || !is_in_bounds(global.2) {
            return None;
        }
        let convert = |val: i32| {
            if val == 0 {
                return (true, 0);
            }
            let index = val.is_positive();
            if index {
                (index, (val) as u16)
            } else {
                (index, (val + size) as u16)
            }
        };
        let converted = (convert(global.0), convert(global.1), convert(global.2));
        Some(NodeLocalPos {
            child: (converted.0 .0, converted.1 .0, converted.2 .0),
            position_in_child: (converted.0 .1, converted.1 .1, converted.2 .1),
        })
    }
}

#[derive(Debug)]
struct BranchNode<T: Debug + Default> {
    children: [[[NodeChild<T>; 2]; 2]; 2], //starting in top back left corner.
                                           // there is no "size: u16" here.
                                           // branch nodes don't actually store their "size".
                                           // Instead, it is found as you progress down from the root
}
impl<T: Debug + Default> BranchNode<T> {
    fn new() -> Self {
        Self {
            children: Default::default(),
        }
    }
    fn get_weak(&self, pos: PosUnsigned, size: u16) -> Option<&T> {
        let (index, pos) = Self::pos_to_index_and_relative(pos, size);
        self.children[index.2 as usize][index.1 as usize][index.0 as usize].get_weak(pos, size / 2)
    }
    fn get_mut_weak(&mut self, pos: PosUnsigned, size: u16) -> Option<&mut T> {
        let (index, pos) = Self::pos_to_index_and_relative(pos, size);
        self.children[index.2 as usize][index.1 as usize][index.0 as usize]
            .get_mut_weak(pos, size / 2)
    }
    fn get_mut_strong(&mut self, pos: PosUnsigned, size: u16, new_t: T) -> &mut T {
        let (index, pos) = Self::pos_to_index_and_relative(pos, size);
        self.children[index.2 as usize][index.1 as usize][index.0 as usize]
            .get_mut_strong(pos, size / 2, new_t)
    }
    fn pos_to_index_and_relative(
        pos: PosUnsigned,
        size: u16,
    ) -> ((usize, usize, usize), PosUnsigned) {
        let index = |val: u16| (val >= size) as usize;
        let index = (index(pos.0), index(pos.1), index(pos.2));
        let relative = (
            (pos.0 - index.0 as u16 * size),
            (pos.1 - index.1 as u16 * size),
            (pos.2 - index.2 as u16 * size),
        );
        ((index.0, index.1, index.2), relative)
    }
}

impl<T: Default + Debug> Default for BranchNode<T> {
    fn default() -> Self {
        let pair: fn() -> [NodeChild<T>; 2] = || [NodeChild::default(), NodeChild::default()];
        let empty = [[pair(), pair()], [pair(), pair()]];
        Self { children: empty }
    }
}

#[cfg(test)]
mod tests {
    use super::OctreeTrait;
    use rstest::*;
    use core_obj::Pos;

    type Inner = u128;
    type Octree = super::Octree<Inner>;

    #[rstest]
    #[case(16)]
    #[case(10)]
    #[case(0)]
    #[case(1)]
    fn create_basic(#[case] size: u16) {
        let new = Octree::new(size);
        let size = size as i32;
        assert_eq!(new.get_weak(&Pos::new(size, size, size)), None);
        assert_eq!(new.get_weak(&Pos::new(-size, -size, size)), None);
        assert_eq!(new.get_weak(&Pos::new(-size, size, -size)), None);
    }

    #[rstest]
    #[case(10, (3,-4,3))]
    #[case(8, (-1,3,0))]
    #[case(12, (14,-10,2))]
    #[case(12, (14,40,200))]
    #[case(0, (140,400,200))]
    fn set_get(#[case] size: u16, #[case] pos: (i32, i32, i32)) {
        let pos_shifted = Pos::new(pos.0, pos.1, pos.2 - 1);
        let pos = Pos::new(pos.0,pos.1,pos.2);
        let mut new = Octree::new(size);
        *new.get_mut_strong(&pos) = 90;
        assert_eq!(new.get_weak(&pos), Some(&90));
        assert_eq!(new.get_mut_weak(&pos), Some(&mut 90));
        assert_eq!(new.get_mut_strong(&pos), &mut 90);
        assert_eq!(new.get_weak(&pos_shifted), None);
        *new.get_mut_strong(&pos) = 90;
    }
    #[rstest]
    #[case(24, [(2,-2,5), (2,-1,5), (1,-2,5)])]
    #[case(18, [(12,-42,54), (0,0,0), (0,1,0)])]
    #[case(4, [(1,-0,2), (0,-1,0), (0,1,0)])]
    #[case(4, [(1,-0,2), (5,-1,0), (20,1,0)])]
    fn set_get_many(#[case] size: u16, #[case] pos: [(i32, i32, i32); 3]) {
        assert_ne!(pos[0], pos[1]);
        assert_ne!(pos[1], pos[2]); // overriding is a different test.
        let pos0 = Pos::new(pos[0].0,pos[0].1,pos[0].2);
        let pos1 = Pos::new(pos[1].0,pos[1].1,pos[1].2);
        let pos2 = Pos::new(pos[2].0,pos[2].1,pos[2].2);

        let mut new = Octree::new(size);
        *new.get_mut_strong(&pos0) = 0;
        assert_eq!(new.get_weak(&pos0), Some(&0));
        *new.get_mut_strong(&pos1) = 1;
        assert_eq!(new.get_weak(&pos1), Some(&1));
        *new.get_mut_strong(&pos2) = 2;
        assert_eq!(new.get_weak(&pos2), Some(&2));
    }
    #[rstest]
    #[case(16, (17,-18,-3))]
    #[case(32, (17,-18,-3))]
    #[case(32, (0,-0,0))]
    fn overriding(#[case] size: u16, #[case] pos: (i32, i32, i32)) {
        let mut new = Octree::new(size);
        let pos = Pos::new(pos.0,pos.1,pos.2);
        *new.get_mut_strong(&pos) = 12;
        assert_eq!(new.get_weak(&pos), Some(&12));
        *new.get_mut_strong(&Pos::new(100, 100, 100)) = 32; //random one.
        assert_eq!(new.get_weak(&pos), Some(&12));
        *new.get_mut_strong(&pos) = 13;
        assert_eq!(new.get_weak(&pos), Some(&13));
    }
}
