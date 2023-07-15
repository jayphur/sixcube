use std::fmt::Debug;

pub use dim_info::DimTypeInfo;
pub use voxel_info::VoxelInfo;

mod dim_info;
mod voxel_info;

type DimType = crate::dim::DimType;
type VoxelType = crate::voxel::VoxelType;

trait Build<T>: Default + Debug {
    fn build(self) -> T;
}
trait Override<T>: Default + Debug {
    fn edit(&self, t: T) -> T;
}

#[derive(Debug, Clone)]
pub enum Attachment {
    NewVoxel(VoxelInfo),
    OverrideVoxel(VoxelInfo),

    NewDim(DimTypeInfo),
    OverrideDim(DimTypeInfo),
}
