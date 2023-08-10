use sc_prelude::*;

use super::chunk::Chunk;

mod octree;

#[derive(Debug, Default)]
pub(crate) struct ChunkSpace<V: Debug + Default>{
    grid: octree::Octree<Chunk<V, 16>>
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
            println!("Trying to make a tree that includes {}", must_include);
            let mut new = Octree::new(must_include);
            println!("setting {:?} to 90 on the tree", pos);
            *new.get_mut_strong(pos) = 90;
            let pos_2 = (pos.0 + 2, -pos.1, pos.2);
            //TODO: do bs with pos_2

            assert_eq!(new.get_weak(pos), Some(&90));
            assert_eq!(new.get_mut_weak(pos), Some(&mut 90));
            assert_eq!(new.get_mut_strong(pos), &mut 90);
        };
        //within bounds
        single(16, (-12, 10, -2));
        single(18, (-18, 0, -12));
        single(18, (-0, 0, 0));
        single(0, (-0, 0, 0));
        single(1, (-1, 0, -1));
        //outside bounds (will need to grow)
        //TODO
    }
}