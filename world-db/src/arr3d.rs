use crate::{PosU, CHUNK_SIZE};
use prelude::*;

///W will have to be n^3, sorry.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Arr3d<T>(pub(crate) [[[T;CHUNK_SIZE];CHUNK_SIZE];CHUNK_SIZE]) 
where T: Clone + Debug + Default + PartialEq;

impl<T> Arr3d<T>
where T: Clone + Debug + Default + PartialEq
{
    pub fn get(&self, pos: PosU) -> &T{
        &self.0[pos.0][pos.1][pos.2]
    }
    pub fn get_mut(&mut self, pos: PosU) -> &mut T{
        &mut self.0[pos.0][pos.1][pos.2]
    }
}