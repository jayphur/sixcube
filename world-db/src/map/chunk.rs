use std::hash::RandomState;

use rustc_hash::FxHashMap;

use core_obj::{DataContainer, PosU, VoxelId};

use crate::{ChunkLocalPos, ChunkPos, ReadChunkTrait, WriteChunkTrait};
use crate::map::arr3d::Arr3d;

#[derive(Clone, Debug,Default)]
pub struct ChunkData{
	pub(crate) voxels: Arr3d<Option<VoxelId>>,
	pub(crate) voxel_data: FxHashMap<ChunkLocalPos, DataContainer>,
}

pub struct WriteChunk<'a>{
	pub guard: chashmap_async::WriteGuard<'a, ChunkPos,ChunkData,RandomState>
}

impl<'a> WriteChunkTrait<'a> for WriteChunk<'a> {
	fn get_mut(&mut self, pos: PosU) -> &mut Option<VoxelId> {
		self.guard.voxels.get_mut(pos)
	}
}

pub struct ReadChunk<'a>{
	pub guard: chashmap_async::ReadGuard<'a, ChunkPos,ChunkData,RandomState>
}

impl<'a> ReadChunkTrait<'a> for ReadChunk<'a> {
	fn get(&self, pos: PosU) -> &Option<VoxelId> {
		self.guard.voxels.get(pos)
	}
}