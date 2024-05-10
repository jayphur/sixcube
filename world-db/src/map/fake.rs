use core_obj::PosU;

use crate::map::chunk::ChunkData;

pub fn test_chunk(mut seed: usize) -> ChunkData{
	let numbers = [0,2,3,4,1,5,96,4,62,5,35,23,23,53,64,93,25,25,73,32,5,123,65,245,98,65,10,101];
	let mut data = ChunkData::default();
	for x in 0.. 128usize{
		let mut pos = PosU((x*seed*97)%16,(x*seed*11)%16,(x*seed*13)%16);
		if (x+seed)%3%2==0{
			for i in 0..x{
				*data.voxels.get_mut(
					PosU( (pos.0 + i ) % 16,pos.1,pos.2)
				) = Some(core_obj::fake::test_voxel(numbers[x*seed%numbers.len()]))
			}
		} else {
			*data.voxels.get_mut(pos) = Some(core_obj::fake::test_voxel(numbers[x*seed%numbers.len()]))
		}
	}
	data
}