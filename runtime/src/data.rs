use serde::{Deserialize, Serialize};

use core_obj::Data;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DataContainer{
	pub data: Vec<Data>,
}