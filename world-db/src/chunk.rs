use std::iter;

use prelude::*;
use core_obj::{TypeId, Data, Voxel, Pos};

use crate::{ChunkTrait, CwPos};
use super::CHUNK_SIZE;

#[derive(Debug)]
pub(crate) struct Chunk<T: TypeId,D: Data>{
    voxels: [[[Option<Voxel<T,D>>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}
impl<T: TypeId,D: Data> ChunkTrait<T,D> for Chunk<T,D>{
    fn contains_attr(&self, attr: <T as TypeId>::AttrId) -> bool {
        todo!()
    }

    fn tell<M: Send>(&self, pos: Pos, msg: M) {
        todo!()
    }

    fn get(&self, pos: Pos) -> &Option<Voxel<T,D>> {
        todo!()
    }

    fn iter_voxel<'a>(&'a self, cw_pos: CwPos) -> impl Iterator<Item=(&Voxel<T,D>, Pos)>
    where D: 'a, T: 'a {
        iter::empty()
    }

    fn iter_voxel_mut<'a>(&'a mut self, cw_pos: CwPos) -> impl Iterator<Item=(&mut Voxel<T,D>, Pos)>
    where D: 'a, T: 'a {
        iter::empty()
    }

}
impl<T: TypeId,D: Data> Default for Chunk<T,D>{
    fn default() -> Self {
        let arr_1d: [_; CHUNK_SIZE] = std::array::from_fn(|_| None);
        let arr_2d: [[_; CHUNK_SIZE]; CHUNK_SIZE] = std::array::from_fn(|_| arr_1d.clone());
        let arr_3d: [[[_; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] = std::array::from_fn(|_| arr_2d.clone());
        
        Self { 
            voxels: arr_3d 
        }
    }
}

lazy_static! {
    static ref ALL_POS: Vec<Pos> = {
        let size = CHUNK_SIZE as i32;
        let mut vec: Vec<_> = Vec::with_capacity(CHUNK_SIZE.pow(3) as usize);
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    vec.push(Pos::new(x, y, z));
                }
            }
        }
        vec
    };
}

#[cfg(test)]
mod test{

}