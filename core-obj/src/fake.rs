use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Data, Registrar, RuntimeType, Value};

/// For tests
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FakeRegistrar{
	test_name: String
}
impl Registrar for FakeRegistrar{
	type VoxelType = FakeVoxel;

	fn all_voxels(&self) -> &[Self::VoxelType] {
		&[FakeVoxel(0),FakeVoxel(1),FakeVoxel(2),FakeVoxel(3)]
	}

	fn voxel_name(&self, voxel: &Self::VoxelType) -> Option<&String> {
		Some(&self.test_name)
	}

	fn voxel_default_data(&self, voxel: &Self::VoxelType) -> Option<&Self::DataContainer> {
		todo!()
	}

	fn find_voxel_by_name(&self, name: String) -> &Self::VoxelType {
		todo!()
	}

	type AttrType = FakeAttrType;

	fn all_attr(&self) -> &[Self::AttrType] {
		todo!()
	}

	fn attr_name(&self, attr: &Self::AttrType) -> Option<&String> {
		todo!()
	}

	fn attr_default(&self, attr: &Self::AttrType) -> Value {
		todo!()
	}

	fn find_attr_by_name(&self, name: String) -> &Self::AttrType {
		todo!()
	}

	type DataContainer = Vec<Data>;
}

#[derive(Debug,Clone,Copy,Serialize,Deserialize, PartialEq,Eq)]
pub struct FakeVoxel(pub u8);

impl RuntimeType for FakeVoxel {

}

#[derive(Debug,Clone,Copy,Serialize,Deserialize, PartialEq,Eq)]
pub struct FakeAttrType(pub u8);

impl RuntimeType for FakeAttrType {

}