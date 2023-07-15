use std::{fmt::Debug};

use super::voxel::Voxel;

/// Dimension
pub struct Dim<Map> where
Map: MapTrait<Voxel>,
{
    map: Map,
}

pub trait MapTrait<Vox>: Default + Debug + Clone{

}

