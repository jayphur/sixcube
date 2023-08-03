use sc_prelude::Debug;

pub trait Voxel: Default + Debug + Clone{}
pub trait Entity: Default + Debug + Clone{}