use std::cmp::PartialEq;
use std::future::Future;
use std::io::ErrorKind;
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rustc_hash::FxHashMap;
use tokio::sync::RwLock;

use core_obj::PosU;
use prelude::*;

use crate::{ChunkPos, MapTrait};
use crate::disk::encoding::{decode_chunk, encode_chunk};
use crate::disk::lookup_table::LookupTable;
use crate::disk::region::{get_table, RegionError};
use crate::map::chunk::ChunkData;

mod lookup_table;
mod encoding;
mod region;
mod file;
const REGION_DIRECTORY_NAME: &str = "regions";

lazy_static!{
    pub static ref BINCODE_OPTIONS: bincode::DefaultOptions = {
		bincode::options()
	};
}

/// Abstracts logistics of caching lookup tables, updating lookup tables, etc.
pub struct MapFile<M: MapTrait>{
	regions: RwLock<FxHashMap<RegionPos, RegionFileInfo>>,
	region_dir: Arc<PathBuf>,
	dir: Arc<PathBuf>,
	__map: PhantomData<M>
}
///////////////// Private
impl<M: MapTrait> MapFile<M> {
	async fn init_region(&self, pos: RegionPos) -> Result<()>{
		if self.regions.read().await.iter().find(|&(&p, _)| pos == pos).is_some(){
			return Ok(());
		}

		let path = Arc::new(PathBuf::from(to_region_file_name(pos)));
		let table = match region::get_table(&path).await{
			Ok(opt) => Some(opt),
			Err(RegionError::FileMissing) => None,
			Err(RegionError::ContentsCorrupted) => return Err(anyhow!("Region File corrupted")),
			Err(RegionError::Other(other)) => return Err(anyhow!("Encountered error when access region file {}", other))
		};
		let region_file_exists = table.is_some();
		let info = RegionFileInfo{
			pos,
			path,
			table_modified: false,
			table: table.unwrap_or_default(),
		};
		if !region_file_exists{
			region::create_region_file(&info).await?;
		}
		self.regions.write().await.insert(pos,info);
		Ok(())
	}
}
///////////////// Public
impl<M: MapTrait> MapFile<M> {
	pub async fn init(path: Arc<PathBuf>) -> Result<Self>{
		let region_dir = Arc::new(PathBuf::from(path.deref()).join(Path::new("regions")));
		let mut region_map: FxHashMap<RegionPos,RegionFileInfo> = Default::default();

		match tokio::fs::read_dir(region_dir.as_ref()).await{
			Ok(mut read_dir) => {
				while let Some(entry) = read_dir.next_entry().await?{
					let Some(pos) = from_region_file_name(entry.file_name().to_str().unwrap()) else {
						continue;
					};
					let path = Arc::new(entry.path());
					let table = get_table(&path).await?;
					region_map.insert(pos, RegionFileInfo{
						pos,
						path,
						table_modified: false,
						table,
					});
				}
			},
			Err(err) => {
				if let ErrorKind::NotFound = err.kind(){
					 tokio::fs::create_dir(path.as_ref()).await?;
				} else {
					return Err(err.into())
				}
			}
		}

		Ok(Self{
			regions: RwLock::new(region_map),
			region_dir,
			dir: path,
			__map: Default::default(),
		})
	}
	pub async fn read(&self, pos: ChunkPos) -> Result<Option<EncodedChunk>>{
		let region_pos = RegionPos::from(pos);
		let region_local_pos = RegionLocalPos::from(pos);
		let read_guard = self.regions.read().await;
		let Some(info) = read_guard.get(&region_pos) else {
			return Ok(None);
		};
		Ok(region::read(info,region_local_pos.into()).await?)
	}

	pub async fn write(&self, pos: ChunkPos, data: &EncodedChunk) -> Result<()> {
		let region_pos = RegionPos::from(pos);
		let region_local_pos = RegionLocalPos::from(pos);
		let mut write_guard = self.regions.write().await;
		let info = if let Some(info) = write_guard.get_mut(&region_pos){
			info
		} else {
			self.init_region(region_pos).await?;
			write_guard.get_mut(&region_pos).unwrap()
		};
		Ok(region::write(info,region_local_pos.into(),data).await?)
	}
}

#[derive(Default,Debug,Copy, Clone,Hash, Eq, PartialEq)]
pub struct RegionPos(pub i16,pub i16,pub i16);

impl From<ChunkPos> for RegionPos {
	fn from(value: ChunkPos) -> Self {
		let n = |val: i16| {
			if val.is_negative(){
				(val - 15)/16
			} else{
				val/16
			}
		};
		Self(
			n(value.0),
			n(value.1),
			n(value.2)
		)
	}
}

#[derive(Default,Debug,Copy, Clone,Hash)]

struct RegionLocalPos(pub u16,pub u16,pub u16);

impl Into<PosU> for RegionLocalPos {
	fn into(self) -> PosU {
		PosU(self.0 as usize,self.1 as usize,self.2 as usize)
	}
}

impl From<ChunkPos> for RegionLocalPos {
	fn from(value: ChunkPos) -> Self {
		let n = |val: i16| {
			val.rem_euclid(16)
		};
		Self(
			n(value.0) as u16,
			n(value.1) as u16,
			n(value.2) as u16
		)    }
}


#[derive(Default, Debug, Clone)]
struct RegionFileInfo {
	pos: RegionPos,
	path: Arc<PathBuf>,
	table_modified: bool,
	table: LookupTable,
}

fn to_region_file_name(pos: RegionPos) -> String{
	format!("{},{},{}.dat",pos.0,pos.1,pos.2)
}

fn from_region_file_name(mut name: &str) -> Option<RegionPos>{
	if name.len() < 9 { return None; }
	let (name,_) = name.split_at(name.len() - 4 );
	println!("name = {name}");
	let first_comma = name.find(",")?;
	let (_,after_first) = name.split_at(first_comma + 1);
	let second_comma = after_first.find(",")? + first_comma + 1;
	let x = name[0..first_comma].parse::<i16>().ok()?;
	let y = name[(first_comma + 1)..second_comma].parse::<i16>().ok()?;
	let z = name[(second_comma + 1)..name.len()].parse::<i16>().ok()?;
	Some(RegionPos(x,y,z))
}


pub struct EncodedChunk(Vec<u8>);

impl EncodedChunk {
	pub fn encode(data: &ChunkData) -> Self{
		Self(encode_chunk(data))
	}
	pub fn decode(&self) -> Result<ChunkData>{
		decode_chunk(&self.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn region_file_name() {
		assert_eq!(Some(RegionPos(1,2,3)), from_region_file_name(&to_region_file_name(RegionPos(1,2,3))));
		assert_eq!(Some(RegionPos(41,232,3)), from_region_file_name(&to_region_file_name(RegionPos(41,232,3))));
		assert_eq!(None, from_region_file_name(""));
		assert_eq!(None, from_region_file_name(".dat"));
		assert_eq!(None, from_region_file_name("1,2.dat"));
		assert_eq!(None, from_region_file_name("5,,.dat"));

	}
}