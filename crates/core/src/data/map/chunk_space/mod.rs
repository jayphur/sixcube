use ndarray::{ArrayBase, Array, Array3};
use sc_prelude::*;

mod octree;

#[derive(Debug, Default)]
pub(crate) struct ChunkSpace<V: Debug + Default + Clone, const SIZE: usize>{
    grid: octree::Octree<Chunk<V, SIZE>>
}
#[derive(Debug)]
pub(crate) struct Chunk<V: Default + Debug + Clone, const SIZE: usize>{
    voxels: Array3<V>, 
}
impl<V: Default + Debug + Clone, const SIZE: usize> Default for Chunk<V, SIZE>{
    fn default() -> Self {
        Self {
            voxels: Array3::<V>::default((SIZE,SIZE,SIZE)),
        }
    }
}
trait GrowingOctree<T>: Debug + Default{
    fn new(size: i16) -> Self;
    fn get_weak(&self, pos: (i16,i16,i16)) -> Option<&T>;
    /// Will not create a new one if this position doesn't exist.
    fn get_mut_weak(&mut self, pos: (i16,i16,i16)) -> Option<&mut T>;
    /// Will create a new one if this position doesn't exist.
    fn get_mut_strong(&mut self, pos: (i16,i16,i16)) -> &mut T;
}





////////////////////////////////////////




#[cfg(test)]
mod tests{ 
    use super::GrowingOctree;
    use rstest::*;

    type Inner = u128;
    type Octree = super::octree::Octree<Inner>;

    #[rstest]
    #[case(16)]
    #[case(10)]
    #[case(-10)]
    #[case(0)]
    #[case(1)]
    fn create_basic(#[case] min_size: i16) {
        let new = Octree::new(min_size);
        assert_eq!(new.get_weak((min_size,min_size,min_size)), None);
        assert_eq!(new.get_weak((-min_size,-min_size,min_size)), None);
        assert_eq!(new.get_weak((-min_size,min_size,-min_size)), None);
    }

    #[rstest]
    #[case(10, (3,-4,3))] //FIXME: tree is a node too shallow
    #[case(8, (-1,3,0))]
    #[case(12, (14,-10,2))]
    #[case(12, (14,40,200))]
    #[case(0, (140,400,200))]
    fn set_get(#[case] min_size: i16, #[case] pos: (i16,i16,i16)){
        let mut new = Octree::new(min_size);
        *new.get_mut_strong(pos) = 90;
        assert_eq!(new.get_weak(pos), Some(&90));
        assert_eq!(new.get_mut_weak(pos), Some(&mut 90));
        assert_eq!(new.get_mut_strong(pos), &mut 90);
        assert_eq!(new.get_weak((pos.0,pos.1,pos.2 - 1)), None);
        *new.get_mut_strong(pos) = 90;
    }
    #[rstest]
    #[case(24, [(2,-2,5), (2,-1,5), (1,-2,5)])]
    #[case(18, [(12,-42,54), (0,0,0), (0,1,0)])]
    #[case(4, [(1,-0,2), (0,-1,0), (0,1,0)])]
    #[case(4, [(1,-0,2), (5,-1,0), (20,1,0)])]
    fn set_get_many(#[case] size:i16, #[case] pos: [(i16,i16,i16);3]){
        assert_ne!(pos[0],pos[1]);
        assert_ne!(pos[1],pos[2]); // overriding is a different test.
        let mut new = Octree::new(size);
        *new.get_mut_strong(pos[0]) = 0;
        assert_eq!(new.get_weak(pos[0]),Some(&0));
        *new.get_mut_strong(pos[1]) = 1;
        assert_eq!(new.get_weak(pos[1]),Some(&1));
        *new.get_mut_strong(pos[2]) = 2;
        assert_eq!(new.get_weak(pos[2]),Some(&2));
    }
    #[rstest]
    #[case(16, (17,-18,-3))]
    #[case(32, (17,-18,-3))]
    #[case(32, (0,-0,0))]
    fn overriding(#[case] size:i16, #[case] pos: (i16,i16,i16)){
        let mut new = Octree::new(size);
        *new.get_mut_strong(pos) = 12;
        assert_eq!(new.get_weak(pos), Some(&12));
        *new.get_mut_strong((100,100,100)) = 32; //random one.
        assert_eq!(new.get_weak(pos), Some(&12));
        *new.get_mut_strong(pos) = 13;
        assert_eq!(new.get_weak(pos), Some(&13));
    }
}
