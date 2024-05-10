use serde::{Deserialize, Serialize};

use prelude::*;

use crate::PosU;

///Simple 3d array wrapper
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Arr3d<T>(pub(crate) [[[T;crate::CHUNK_USIZE];crate::CHUNK_USIZE];crate::CHUNK_USIZE])
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


