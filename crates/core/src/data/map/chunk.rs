use lazy_static::lazy_static;
use sc_prelude::*;

use crate::pos::Pos;
use crate::pos::RelativePos;

use super::ChunkTrait;
use crate::CHUNK_SIZE as SIZE;
use crate::CHUNK_SIZE as SIZE_U;

#[derive(Debug)]
pub(crate) struct Chunk<T: Default + Debug + Clone + Send, const S: usize> {
    voxels: [[[T; S]; S]; S],
}
impl<T: Default + Debug + Clone + Send, const S: usize> Default for Chunk<T, S> {
    fn default() -> Self {
        let voxels = [[[(); S]; S]; S].map(|s| s.map(|s| s.map(|_| T::default()))); // god awful default initialization.
        Self { voxels }
    }
}

impl<T: Default + Debug + Clone + Send> ChunkTrait<T> for Chunk<T, SIZE_U> {
    fn new() -> Self {
        Self::default()
    }

    fn get(&self, pos: RelativePos) -> &T {
        let pos: (usize, usize, usize) = pos.try_tuple().unwrap();
        &self.voxels[pos.2][pos.1][pos.0]
    }

    fn get_mut(&mut self, pos: RelativePos) -> &mut T {
        let pos: (usize, usize, usize) = pos.try_tuple().unwrap();
        &mut self.voxels[pos.2][pos.1][pos.0]
    }

    fn all_pos() -> &'static Vec<RelativePos> {
        &ALL_POS
    }
}

lazy_static! {
    static ref ALL_POS: Vec<RelativePos> = {
        let size = SIZE as i16;
        let mut vec: Vec<_> = Vec::with_capacity(SIZE.pow(3) as usize);
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    vec.push(RelativePos::new((x, y, z)));
                }
            }
        }
        vec
    };
}
