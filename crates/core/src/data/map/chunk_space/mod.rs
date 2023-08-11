use sc_prelude::*;

mod octree;

#[derive(Debug, Default)]
pub(crate) struct ChunkSpace<V: Debug + Default>{
    grid: octree::Octree<Chunk<V, 16>>
}
#[derive(Default, Debug)]
pub(crate) struct Chunk<V, const S: usize>{
    _v: PhantomData<V>,
}
trait GrowingOctree<T>: Debug + Default{
    fn new(must_include: i16) -> Self;
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
    fn set_get(#[case] min_size: i16, #[case] pos: (i16,i16,i16)){
        let mut new = Octree::new(min_size);
        *new.get_mut_strong(pos) = 90;
        assert_eq!(new.get_weak(pos), Some(&90));
        assert_eq!(new.get_mut_weak(pos), Some(&mut 90));
        assert_eq!(new.get_mut_strong(pos), &mut 90);
        assert_eq!(new.get_weak((pos.0,pos.1,pos.2 - 1)), None);
        *new.get_mut_strong(pos) = 90;
    }
}
