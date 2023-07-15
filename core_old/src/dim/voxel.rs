use crate::dim;
use core::fmt::Debug;

mod comp;
mod voxel_type;

type Cmp = comp::VoxelCmp;
pub type VoxelType = voxel_type::VoxelType;

#[derive(Debug, Default, Clone, Copy)]
pub enum Voxel {
    #[default]
    Empty,
    Voxel(Cmp),
}
impl Voxel {
    fn type_str(&self) -> &'static str {
        match &self {
            Self::Empty => &"Empty",
            Self::Voxel(c) => c.type_str(),
        }
    }
}
#[derive(Default, Debug, Clone, Copy)]
pub struct VoxelCmp {
    v_type: VoxelType,
}