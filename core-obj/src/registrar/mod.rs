use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub use attr::*;
pub use attr::{AttrId, AttrInfo};
pub use voxel::*;
pub use voxel::{DataContainer, VoxelId, VoxelInfo};

mod attr;

mod voxel;

/// A type, as part of the user defined plugin system
pub trait RegistrarType: Send + Sync + Debug + Clone + Copy + Serialize + for <'a> Deserialize<'a> + PartialEq {}


#[derive(Debug,Clone,Default)]
pub struct Registrar{
	voxel: voxel::Registrar,
	attr: attr::Registrar,
}

impl Registrar {
	pub fn voxel_name(&self, id: &VoxelId) -> Option<&String>{
		Some(&self.voxel.info(id)?.name)
	}
}