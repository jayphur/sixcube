use serde::{Deserialize, Serialize};

use core_obj::RuntimeType;

use crate::data::DataContainer;

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VoxelType{
	pub id: u16,
	param: (u8, u8),
}

impl RuntimeType for VoxelType {}

#[derive(Debug, Clone)]
pub struct VoxelInfo{
	pub default_data_container: Option<DataContainer>,
	pub name: String,
}
