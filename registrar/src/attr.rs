use serde::{Deserialize, Serialize};

use core_obj::{RuntimeType, Value};

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AttrType{
	pub id: u16
}

impl RuntimeType for AttrType {}


#[derive(Debug, Clone)]
pub struct AttrInfo{
	pub name: String,
	pub default: Value,
}