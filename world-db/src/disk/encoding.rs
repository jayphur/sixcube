use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::iter;

use bincode::Options;
use itertools::Itertools;
use rustc_hash::{FxHasher, FxHashMap};
use serde::{Deserialize, Serialize};

use core_obj::{DataContainer, VoxelId};
use prelude::*;

use crate::{CHUNK_USIZE, ChunkLocalPos};
use crate::map::arr3d::Arr3d;
use crate::map::chunk::ChunkData;

/// NOTE: cpu bound for sure
pub fn encode_chunk(chunk: &ChunkData) -> Vec<u8>{
	let smaller = SmallerChunk::new(chunk);
	super::BINCODE_OPTIONS.serialize(&smaller).unwrap()
}
/// NOTE: cpu bound for sure
pub fn decode_chunk(bytes: &[u8]) -> Result<ChunkData>{
	let smaller: SmallerChunk = super::BINCODE_OPTIONS.deserialize(bytes)?;
	smaller.to_chunk()
}

#[derive(Debug, Deserialize, Serialize)]
struct SmallerChunk{
	voxels: Arr3dRLE<Option<VoxelId>>,
	data: Vec<(ChunkLocalPos, DataContainer)>, //TODO: remove unnecessary clone with DataContainer (requires manual Deserialize implementation)
}

impl SmallerChunk {
	fn new(chunk: &ChunkData) -> Self{
		Self{
			voxels: Arr3dRLE::from_arr3d(&chunk.voxels),
			data: chunk.voxel_data.iter().map(|(&k,v)|(k,v.clone())).collect_vec(),
		}
	}
	fn to_chunk(self) -> Result<ChunkData>{
		let hasher: BuildHasherDefault<FxHasher> = BuildHasherDefault::default();
		let mut voxel_data: HashMap<ChunkLocalPos, DataContainer, _> = FxHashMap::with_capacity_and_hasher(self.data.len(), hasher);
		voxel_data.extend(self.data.iter().map(|(pos,val)|{(*pos,val.clone())}));
		Ok(ChunkData{
			voxels: self.voxels.into_arr3d()?,
			voxel_data,
		})
	}
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Arr3dRLE<T> where T: Clone + Debug + Default + Serialize + PartialEq{
	data: Vec<(u8,usize)>,
	key: Vec<(u8,T)>,
}

impl<T> Arr3dRLE<T> where T: Clone + Debug + Default + Serialize + PartialEq {
	pub fn into_arr3d(self) -> Result<Arr3d<T>>{

		let mut full_length = self.data
			.into_iter()
			.flat_map(|(id, len)|{
				iter::repeat(id).take(len)
			})
			.map(|id|{
				let Some((_,val)) = self.key.iter().find(|(key_id,val)| *key_id == id) else {
					return Err(anyhow!("Arr3dRLE missing key for id: {}", id))
				};
				return Ok(val.clone())
			});
		let arr3d = Arr3d(std::array::try_from_fn::<_, { CHUNK_USIZE },_>(|i|{
			std::array::try_from_fn::<_, { CHUNK_USIZE },_>(|i|{
				std::array::try_from_fn::<_, { CHUNK_USIZE },_>(|i|{
					let Some(result) = full_length.next() else {
						return Err(anyhow!("Incomplete Arr3dRLE, unable to convert to Arr3d. (Data vector did not contain the sufficient 16^3 entries)", ));
					};
					result
				})
			})
		})?);
		Ok(arr3d)
	}
	fn from_arr3d(other: &Arr3d<T>) -> Self { //TODO: hacked garbage, but quarantined hacked garbage lmfao
		let mut key: Vec<(u8,T)> = Vec::with_capacity(2);
		let flat = other.0
			.flatten()
			.flatten();
		let mut next = flat.iter();
		next.next();
		let mut count: usize = 1;
		let data = flat
			.into_iter()
			.filter_map(|val|{
				if Some(val) == next.next(){
					count += 1;
					None
				} else {
					let mut count_old = 1;
					std::mem::swap(&mut count_old, &mut count);
					Some((val, count_old))
				}
			})
			.map(|(val, len)|{
				let id = match key.iter().find(|(_, key)| *key == *val){
					Some((id, _)) => {
						*id
					},
					None => {
						let id = key.len() as u8;
						key.push((id, val.clone()));
						id
					},
				};
				(id, len)

			}).collect::<Vec<(u8,usize)>>();

		Arr3dRLE{
			data,
			key,
		}
	}
}

#[cfg(test)]
mod tests{
	use prelude::Result;

	use crate::PosU;

	use super::{Arr3d, Arr3dRLE};

	#[test]
	fn round_trip_rle_arr3d() -> Result<()>{
		let mut starting: Arr3d<i32> = Arr3d::default();
		*starting.get_mut(PosU(15, 2, 13)) = 23;
		*starting.get_mut(PosU(0, 4, 13)) = -324;
		*starting.get_mut(PosU(14, 5, 13)) = 945;
		*starting.get_mut(PosU(15, 4, 13)) = 26894;

		*starting.get_mut(PosU(5, 1, 15)) = 78;
		*starting.get_mut(PosU(2, 0, 2)) = -32;
		*starting.get_mut(PosU(4, 1, 5)) = 95;
		*starting.get_mut(PosU(7, 8, 3)) = 2894;

		let converted: Arr3dRLE<i32> = Arr3dRLE::from_arr3d(&starting);
		let back: Arr3d<i32> = converted.into_arr3d()?;
		assert_eq!(back, starting);
		Ok(())
	}

}