use serde::{Deserialize, Serialize};

use crate::Value;

use super::RegistrarType;

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AttrId {
	pub id: u16
}

impl RegistrarType for AttrId {}


#[derive(Debug, Clone)]
pub struct AttrInfo{
	pub name: String,
	pub default: Value,
}

#[derive(Debug,Clone,Default)]
pub struct Registrar {

}