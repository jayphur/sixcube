use crate::VoxelId;

pub fn fake_voxel(n: u16) -> VoxelId{
	VoxelId{
		id: n,
		variant: 0,
	}
}