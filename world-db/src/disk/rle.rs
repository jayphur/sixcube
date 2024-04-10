use std::fmt::Debug;
use std::iter;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

use prelude::*;

use crate::arr3d::Arr3d;
use crate::CHUNK_SIZE;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arr3dRLE<T> where T: Clone + Debug + Default + Serialize + PartialEq{
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
		let arr3d = Arr3d(std::array::try_from_fn::<_, CHUNK_SIZE,_>(|i|{
			std::array::try_from_fn::<_, CHUNK_SIZE,_>(|i|{
				std::array::try_from_fn::<_, CHUNK_SIZE,_>(|i|{
					let Some(result) = full_length.next() else {
						return Err(anyhow!("Incomplete Arr3dRLE, unable to convert to Arr3d. (Data vector did not contain the sufficient 16^3 entries)", ));
					};
					result
				})
			})
		})?);
		Ok(arr3d)
	}
}


impl<T> From<Arr3d<T>> for Arr3dRLE<T>
	where T: Clone + Debug + Default + Serialize + PartialEq
{
	fn from(other: Arr3d<T>) -> Self { //TODO: hacked garbage, but quarantined hacked garbage lmfao
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
mod test{
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

		let converted: Arr3dRLE<i32> = starting.clone().into();
		let bin = bincode::serialize(&converted)?;
		println!("Bin length is {} bytes",bin.len());
		let bin: Arr3dRLE<i32> = bincode::deserialize(&bin)?;
		let back: Arr3d<i32> = bin.into_arr3d()?;
		assert_eq!(back, starting);
		Ok(())
	}

}