use super::{Build, Override, VoxelType};

#[derive(Default, Debug, Clone)]
pub struct VoxelInfo {}

impl Build<VoxelType> for VoxelInfo {
    fn build(self) -> VoxelType {
        VoxelType {}
    }
}

impl Override<VoxelType> for VoxelInfo {
    fn edit(&self, t: VoxelType) -> VoxelType {
        t
    }
}
