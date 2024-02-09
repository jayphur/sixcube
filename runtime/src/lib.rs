use fxhash::FxHashMap;

use core_obj::Value;

use crate::attr::{AttrInfo, AttrType};
use crate::data::DataContainer;
use crate::voxel::{VoxelInfo, VoxelType};

mod voxel;
mod attr;
mod data;

#[derive(Debug, Clone)]
pub struct Runtime{
	all_voxel: Vec<VoxelType>,
	voxel: FxHashMap<VoxelType, VoxelInfo>,

	all_attr: Vec<AttrType>,
	attr: FxHashMap<AttrType, AttrInfo>,
}

impl Runtime {
	pub fn new() -> Self{
		let voxel: FxHashMap<VoxelType, VoxelInfo> = FxHashMap::default();
		let attr: FxHashMap<AttrType, AttrInfo> = FxHashMap::default();

		Self{
			all_voxel: voxel.keys().map(|c|c.clone()).collect(),
			voxel,
			all_attr: attr.keys().map(|c|c.clone()).collect(),
			attr,
		}
	}
}

impl core_obj::Runtime for Runtime{
	type VoxelType = VoxelType;

	fn all_voxels(&self) -> &[VoxelType] {
		&self.all_voxel
	}

	fn voxel_name(&self, voxel: &Self::VoxelType) -> Option<&String> {
		Some(&self.voxel.get(&voxel)?.name)
	}

	fn voxel_default_data(&self, voxel: &VoxelType) -> Option<&DataContainer> {
		Option::from(&self.voxel.get(&voxel)?.default_data_container)
	}

	fn find_voxel_by_name(&self, name: String) -> &VoxelType {
		todo!()
	}

	type AttrType = AttrType;

	fn all_attr(&self) -> &[AttrType] {
		&self.all_attr
	}

	fn attr_name(&self, attr: &Self::AttrType) -> Option<&String> {
		Some(&self.attr.get(attr)?.name)
	}


	fn attr_default(&self, attr: &AttrType) -> Value {
		self.attr.get(attr).unwrap().default
	}

	fn find_attr_by_name(&self, name: String) -> &AttrType {
		todo!()
	}

	type DataContainer = DataContainer;
}