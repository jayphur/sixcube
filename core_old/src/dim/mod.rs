use std::fmt::Debug;

use self::map::Map;

pub(crate) mod chunk;
pub(crate) mod map;

pub const CHUNK_SIZE: usize = 40;

pub type Voxel = crate::voxel::Voxel;

pub struct Dim {
    map: Map<CHUNK_SIZE, { CHUNK_SIZE * CHUNK_SIZE }>,
    dim_type: DimType,
}
impl Dim {}

pub(crate) trait DimMap: Debug {}

pub(crate) struct DimType {}
// what Dim needs a voxel to do
pub(crate) trait VoxelTrait: Debug + Default + Clone {}
