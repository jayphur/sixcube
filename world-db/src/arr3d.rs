use serde::{Deserialize, Serialize};

use prelude::*;

use crate::PosU;

///Simple 3d array wrapper
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Arr3d<T>(pub(crate) [[[T;crate::CHUNK_SIZE];crate::CHUNK_SIZE];crate::CHUNK_SIZE])
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



#[cfg(test)]
mod tests {
	use core_obj::fake::FakeVoxel;

	use super::*;

	#[test]
    fn serialize_and_deserialize_bincode_u8() {
        let mut arr3d: Arr3d<u8> = Arr3d::default();
        *arr3d.get_mut(PosU(1,2,5)) = 12;
        *arr3d.get_mut(PosU(15,4,0)) = 18;
        *arr3d.get_mut(PosU(3,7,5)) = 49;
        let bin = bincode::serialize(&arr3d).unwrap();
        let de_ser: Arr3d<u8> = bincode::deserialize(&bin).unwrap();
        assert_eq!(arr3d,de_ser)
    }
    #[test]
    fn serialize_and_deserialize_bincode_fake_voxel() {
        let mut arr3d: Arr3d<Option<FakeVoxel>> = Arr3d::default();
        *arr3d.get_mut(PosU(1,2,5)) = Some(FakeVoxel(12));
        *arr3d.get_mut(PosU(15,4,0)) = Some(FakeVoxel(58));
        let bin = bincode::serialize(&arr3d).unwrap();
        let de_ser: Arr3d<Option<FakeVoxel>> = bincode::deserialize(&bin).unwrap();
        assert_eq!(arr3d,de_ser);
    }
}