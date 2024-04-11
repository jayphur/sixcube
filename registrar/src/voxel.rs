use serde::{Deserialize, Serialize};

use core_obj::RegistrarType;

use crate::data::DataContainer;

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VoxelType{
	pub id: u16,
	param: [u8;2],
}

impl RegistrarType for VoxelType {}

#[derive(Debug, Clone)]
pub struct VoxelInfo{
	pub default_data_container: Option<DataContainer>,
	pub name: String,
}
