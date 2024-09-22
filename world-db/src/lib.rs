#![feature(slice_flatten)]
#![feature(array_try_from_fn)]
extern crate core;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use core_obj::{PosU, VoxelId};
use prelude::*;
use prelude::thiserror::Error;

pub const CHUNK_USIZE: usize = 16;

pub mod map;
pub mod disk;

#[async_trait]
pub trait MapTrait: Debug{
	type ReadChunk<'a>: ReadChunkTrait<'a> where Self: 'a;
	type WriteChunk<'a>: WriteChunkTrait<'a> where Self: 'a;
	fn new() -> Self;

	async fn read<'a>(&'a self, pos: ChunkPos) -> Result<Option<Self::ReadChunk<'a>>,Error>;
	async fn write<'a>(&'a self, pos: ChunkPos) -> Result<Self::WriteChunk<'a>,Error>;
	async fn clone(&self) -> Self;
}
pub trait ReadChunkTrait<'a>{
	fn get(&self, pos: PosU) -> &Option<VoxelId>;
}
pub trait WriteChunkTrait<'a>{
	fn get_mut(&mut self, pos: PosU) -> &mut Option<VoxelId>;
}

#[derive(Default,Debug,Copy, Clone,Serialize,Deserialize, Hash, Eq, PartialEq)]
pub struct ChunkPos(pub i16,pub i16,pub i16);

#[derive(Default,Debug,Copy, Clone,Serialize,Deserialize, Hash, Eq, PartialEq)]
pub struct ChunkLocalPos(pub u8,pub u8,pub u8);


#[derive(Debug, Error)]
pub enum Error{
	#[error("accessed chunk does not exist")]
	DoesNotExist,
	#[error("misc error")]
	Other(ErrorStruct)
}

#[derive(Debug)]
pub enum ChunkStatus<T: Debug>{
	Loaded(T),
	NotLoaded,
	Nonexistent,
}