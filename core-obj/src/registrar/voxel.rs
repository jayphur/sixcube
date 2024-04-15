use serde::{Deserialize, Serialize};

use crate::Data;

use super::RegistrarType;

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VoxelId {
	pub id: u16,
	pub variant: u16,
}

impl RegistrarType for VoxelId {}

#[derive(Debug, Clone)]
pub struct VoxelInfo{
	pub default_data_container: Option<DataContainer>,
	pub name: String,
	pub possible_variants: u16,
	pub solid: bool
}

impl Default for VoxelInfo {
	fn default() -> Self {
		Self{
			default_data_container: None,
			name: "Voxel".to_string(),
			possible_variants: 1,
			solid: true,
		}
	}
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DataContainer{
	pub data: Vec<Data>,
}

#[derive(Debug,Clone,Default)]
pub(super) struct Registrar{
	ids: Vec<VoxelId>,
	info: Vec<VoxelInfo>
}

impl Registrar {
	pub fn new() -> Self{
		Self{
			ids: vec![],
			info: vec![],
		}
	}
	pub fn add_vox(&mut self, info: VoxelInfo){
		let id= self.info.len() as u16;
		for i in 0..info.possible_variants{
			self.ids.push(VoxelId{
				id,
				variant: i,
			})
		}
		self.info.push(info);
	}
	pub fn info(&self, id: &VoxelId) -> Option<&VoxelInfo>{
		self.info.get(id.id as usize)
	}
}