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

    type Inner = u128;
    type Octree = super::octree::Octree<Inner>;
    #[test]
    fn create(){
        let create = |val: i16| {
            println!("Trying to make a tree of size {}", val);
            let _ = Octree::new(val);
        };
        create(-30);
        create(30);
        create(0);
        create(10000);
        create(2);
    }
    #[test]
    fn create_set_then_get(){
        let single = |must_include: i16, pos: (i16,i16,i16)| {
            println!("\nTrying to make a tree that includes {}", must_include);
            let mut new = Octree::new(must_include);
            println!("setting {:?} to 90 on the tree", pos);
            *new.get_mut_strong(pos) = 90;
            println!("success!");
            let pos_2 = (pos.0 + 2, -pos.1, pos.2);
            assert_eq!(new.get_weak(pos), Some(&90));
            println!("success! get_weak({:?})",pos);
            assert_eq!(new.get_mut_weak(pos), Some(&mut 90));
            println!("success! get_mut_weak({:?})",pos);
            assert_eq!(new.get_mut_strong(pos), &mut 90);
            println!("success! get_mut_strong({:?})",pos);
            assert_eq!(new.get_weak(pos_2), None);
            println!("success! get_weak({:?}) is none",pos_2);
        };
        //within bounds
        
        single(16, (-12, 10, -2));
        single(18, (-18, 0, -12));
        single(-18, (-18, 0, -12));
        single(18, (-0, 0, 0));
        single(18, (-20, 0, -32));
        single(0, (-0, 0, 0));
        single(1, (-1, 0, -1));
        //outside bounds (will need to grow)
        //TODO
        single(16, (-17, 10, -2));
        single(16, (-17, 17, -17));
        single(18, (-33, 0, -12));
        single(18, (-0, 200, 0));
        single(0, (-0, 1, 4));
        single(1, (-2, 0, -1));
    }
}