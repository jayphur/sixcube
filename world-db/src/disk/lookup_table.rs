use std::iter;

use bincode::Options;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use prelude::*;

use super::BINCODE_OPTIONS;

pub const LOOKUP_TABLE_BYTE_SIZE: usize = {((16*16*16)*64 + 64 + (16*16*16)*16) / 8 + 16 + 16}; //first + 16 => end, second + 16 => length
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// In bytes.
pub struct LookupTable{
	pub start: Vec<u64>,
	pub padding: Vec<u16>,
	pub end: u64,
}

impl Default for LookupTable {
	fn default() -> Self {
		Self{
			start: iter::repeat(0).take(16*16*16).collect_vec(),
			padding: iter::repeat(0).take(16*16*16).collect_vec(),
			end: 0,
		}
	}
}

impl LookupTable {
	pub(crate) fn start(&self, u: usize) -> u64{
		self.start[u]
	}

	///w\\o padding, use `length_of_with_padding` for w\\ padding
	pub(crate) fn length_of(&self, u: usize) -> u64{
		self.length_of_with_padding(u) - self.padding[u] as u64
	}
	fn length_of_with_padding(&self, u: usize) -> u64{
		if u >= self.start.len() - 1{
			self.end - self.start[u]
		} else {
			self.start[u+1] - self.start[u]
		}
	}

	///makes sure this index can fit this amount (or greater), returning the amount that everything got shifted
	pub(crate) fn fit(&mut self, index: usize, amount: u64) -> u64{
		let current_length_full = self.length_of_with_padding(index);
		let current_length = self.length_of(index);

		if amount > current_length_full{
			self.padding[index] = 0;

			let amount = amount - current_length_full;
			//grow
			self.start.iter_mut()
				.skip(index + 1)
				.for_each(|val| *val += amount);
			self.end += amount;
			return amount;
		} else if amount > current_length{
			let extra = amount - current_length;
			self.set_padding(index, self.padding[index] as u64 - extra);
		}
		return 0;

	}

	/// Declare that this section has `amount` bytes of padding in it.
	/// This does **NOT** expand anything.
	pub(crate) fn set_padding(&mut self, index:usize, amount: u64){
		self.padding[index] = amount as u16
	}
	pub(crate) fn to_bytes(&self) -> Vec<u8>{
		let mut table = BINCODE_OPTIONS.serialize(&self).unwrap();
		let mut vec = BINCODE_OPTIONS.serialize(&(table.len() as u64)).unwrap();
		vec.resize(8, 0);
		vec.append(&mut table);
		vec.resize(LOOKUP_TABLE_BYTE_SIZE, 0);
		vec
	}
	pub(crate) fn init_from_bytes(mut slice: &[u8]) -> prelude::Result<Self> {
		let (length_indicator, slice) = slice.split_at(8);
		let len: u64 = BINCODE_OPTIONS.allow_trailing_bytes().deserialize(length_indicator)?;
		let (data, _) = slice.split_at(len as usize);
		let decode: Self = BINCODE_OPTIONS.deserialize(data)
			.with_context(|| format!("Failed to decode bytes (len = {}) when creating lookup table", slice.len()))?;
		// if it's an empty boy, init.
		if decode.padding.len() == 0 || decode.start.len() == 0{
			return Ok(Self::default());
		}
		Ok(decode)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn lookup_table_round_trip() {
		let mut table = LookupTable::default();
		*table.start.get_mut(2).unwrap() = 57;
		*table.start.get_mut(23).unwrap() = 48795;
		*table.start.get_mut(25).unwrap() = 48743295;
		*table.start.get_mut(1).unwrap() = 342;
		let bytes = table.to_bytes();
		assert_eq!(bytes.len(), LOOKUP_TABLE_BYTE_SIZE);
		assert_eq!(LookupTable::init_from_bytes(&bytes).unwrap(),table); // Round Trip

	}
	#[test]
	fn lookup_table_byte_size(){
		let mut table = LookupTable::default();
		assert_eq!( table.start.len(), 16*16*16);
		assert_eq!( table.padding.len(), 16*16*16);
		assert_eq!(table.to_bytes().len(), LOOKUP_TABLE_BYTE_SIZE); // LOOKUP_TABLE_SIZE is accurate
	}

	#[test]
	fn lookup_table_padding() {
		let mut table = LookupTable::default();
		table.fit(0, 100);
		for x in 1..16*16*16 {
			assert_eq!(table.start(x), 100);
		}
		table.fit(0, 200);
		table.set_padding(0, 100);
		assert_eq!(table.length_of(0), 100);
		table.fit(0, 150);
		assert_eq!(table.length_of(0), 150);
		for x in 1..16*16*16 {
			assert_eq!(table.start(x), 200);
		}
		table.fit(0, 250);
		assert_eq!(table.length_of(0), 250);
		for x in 1..16*16*16 {
			assert_eq!(table.start(x), 250);
		}
	}

	#[test]
	fn lookup_table_fit() {
		let mut table = LookupTable::default();
		for x in 0..20 {
			table.fit(x, 10);
		}
		for x in 0..20 {
			table.fit(x, 9);
		}
		for x in 0..20 {
			assert_eq!(table.length_of(x), 10);
		}
		for x in 0..20 {
			assert_eq!(table.start(x), x as u64 *10);
		}
		table.fit(10, 25);
		for x in 0..10 {
			assert_eq!(table.length_of(x), 10);
			assert_eq!(table.start(x), x as u64 *10);
		}
		assert_eq!(table.length_of(10),25);
		for x in 11..20 {
			assert_eq!(table.length_of(x), 10);
			assert_eq!(table.start(x), x as u64 *10 + 15);
		}
		for x in 30..16*16*16-1 {
			assert_eq!(table.length_of(x), 0);
		}
	}

	#[test]
	fn default_lookup_table() {
		let mut table = LookupTable::default();
		assert_eq!(table.padding.len(), 16*16*16);
		assert_eq!(table.start.len(), 16*16*16);
	}

}