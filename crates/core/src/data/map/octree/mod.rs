use sc_prelude::*;
use core::panic;

use super::OctreeTrait;

type GlobalPos = (i16, i16, i16);
type PosUnsigned = (u16, u16, u16);

#[derive(Debug)]
pub struct Octree<T: Default + Debug> {
    root: RootNode<T>,
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
        }
    }
}

impl<T: Default + Debug> Default for Octree<T> {
    fn default() -> Self {
        Self::new(4)
    }
}

impl<T: Default + Debug> OctreeTrait<T> for Octree<T> {
    fn get_weak(&self, pos: (i16, i16, i16)) -> Option<&T> {
        self.root.get_weak(pos)
    }

    fn get_mut_weak(&mut self, pos: (i16, i16, i16)) -> Option<&mut T> {
        self.root.get_mut_weak(pos)
    }

    fn get_mut_strong(&mut self, pos: (i16, i16, i16)) -> &mut T {
        self.root.get_mut_strong(pos)
    }

    fn new(size: u16) -> Self {
        let size = (size as usize).next_power_of_two() as u16;
        Self::new(size)
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

    fn get_mut_strong(&mut self, pos: (u16, u16, u16), size: u16) -> &mut T {
        if size == 0 {
            match self {
                NodeChild::Empty => {
                    *self = NodeChild::Leaf(T::default());
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
                    self.get_mut_strong(pos, size)
                }
                NodeChild::Node(node) => node.get_mut_strong(pos, size),
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
    fn get_weak(&self, pos: (i16, i16, i16)) -> Option<&T> {
        let pos = self.global_pos_to_local(pos)?;
        self.children[pos.child.2 as usize][pos.child.1 as usize][pos.child.0 as usize]
            .get_weak(pos.position_in_child, self.size / 2)
    }

    fn get_mut_weak(&mut self, pos: (i16, i16, i16)) -> Option<&mut T> {
        let pos = self.global_pos_to_local(pos)?;
        self.children[pos.child.2 as usize][pos.child.1 as usize][pos.child.0 as usize]
            .get_mut_weak(pos.position_in_child, self.size / 2)
    }

    fn get_mut_strong(&mut self, pos: (i16, i16, i16)) -> &mut T {
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
            return self.get_mut_strong(pos);
        };
        self.children[pos.child.2 as usize][pos.child.1 as usize][pos.child.0 as usize]
            .get_mut_strong(pos.position_in_child, self.size / 2)
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
        let size = self.size as i16;
        let is_in_bounds = |val: i16| -size <= val && size > val;
        if !is_in_bounds(global.0) || !is_in_bounds(global.1) || !is_in_bounds(global.2) {
            return None;
        }
        let convert = |val: i16| {
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
    fn get_mut_strong(&mut self, pos: PosUnsigned, size: u16) -> &mut T {
        let (index, pos) = Self::pos_to_index_and_relative(pos, size);
        self.children[index.2 as usize][index.1 as usize][index.0 as usize]
            .get_mut_strong(pos, size / 2)
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
        //FIXME: clean this up maybe. it might not be possible because rust's array init shorthand is stupid.
        let empty: fn() -> [NodeChild<T>; 2] = || [NodeChild::default(), NodeChild::default()];
        let empty = [[empty(), empty()], [empty(), empty()]];
        Self { children: empty }
    }
}


#[cfg(test)]
mod tests {
    use super::OctreeTrait;
    use rstest::*;

    type Inner = u128;
    type Octree = super::Octree<Inner>;

    #[rstest]
    #[case(16)]
    #[case(10)]
    #[case(0)]
    #[case(1)]
    fn create_basic(#[case] size: u16) {
        let new = Octree::new(size);
        let size = size as i16;
        assert_eq!(new.get_weak((size, size, size)), None);
        assert_eq!(new.get_weak((-size, -size, size)), None);
        assert_eq!(new.get_weak((-size, size, -size)), None);
    }

    #[rstest]
    #[case(10, (3,-4,3))]
    #[case(8, (-1,3,0))]
    #[case(12, (14,-10,2))]
    #[case(12, (14,40,200))]
    #[case(0, (140,400,200))]
    fn set_get(#[case] size: u16, #[case] pos: (i16, i16, i16)) {
        let mut new = Octree::new(size);
        *new.get_mut_strong(pos) = 90;
        assert_eq!(new.get_weak(pos), Some(&90));
        assert_eq!(new.get_mut_weak(pos), Some(&mut 90));
        assert_eq!(new.get_mut_strong(pos), &mut 90);
        assert_eq!(new.get_weak((pos.0, pos.1, pos.2 - 1)), None);
        *new.get_mut_strong(pos) = 90;
    }
    #[rstest]
    #[case(24, [(2,-2,5), (2,-1,5), (1,-2,5)])]
    #[case(18, [(12,-42,54), (0,0,0), (0,1,0)])]
    #[case(4, [(1,-0,2), (0,-1,0), (0,1,0)])]
    #[case(4, [(1,-0,2), (5,-1,0), (20,1,0)])]
    fn set_get_many(#[case] size: u16, #[case] pos: [(i16, i16, i16); 3]) {
        assert_ne!(pos[0], pos[1]);
        assert_ne!(pos[1], pos[2]); // overriding is a different test.
        let mut new = Octree::new(size);
        *new.get_mut_strong(pos[0]) = 0;
        assert_eq!(new.get_weak(pos[0]), Some(&0));
        *new.get_mut_strong(pos[1]) = 1;
        assert_eq!(new.get_weak(pos[1]), Some(&1));
        *new.get_mut_strong(pos[2]) = 2;
        assert_eq!(new.get_weak(pos[2]), Some(&2));
    }
    #[rstest]
    #[case(16, (17,-18,-3))]
    #[case(32, (17,-18,-3))]
    #[case(32, (0,-0,0))]
    fn overriding(#[case] size: u16, #[case] pos: (i16, i16, i16)) {
        let mut new = Octree::new(size);
        *new.get_mut_strong(pos) = 12;
        assert_eq!(new.get_weak(pos), Some(&12));
        *new.get_mut_strong((100, 100, 100)) = 32; //random one.
        assert_eq!(new.get_weak(pos), Some(&12));
        *new.get_mut_strong(pos) = 13;
        assert_eq!(new.get_weak(pos), Some(&13));
    }
}
