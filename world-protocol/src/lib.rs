//! # Plan
//! There are several "systems" aka visitors. 
//! That specify a pool of chunks/voxels that they want to visit.
//! 
//! - The map is divided into chunks of arbitrary size. (In order to be able to fine tune what the best size is with experimentation.)
//! 
//! Each visitor will visit each chunk via mutex 
//! and whenever it want to know stuff about voxels outside its chunk,
//! it can send a message to query another voxel.
//! 
//! We'll do message (and read), respond phase and apply phase.
//! 
//! ## Consistence
//! I want the updating to be consistent, 
//! the old method of ensuring this (I could think of...) 
//! was the two-step *message & respond phase* and *apply phase*.
//! 
//! Do I think this aligns with the proposed ECS-ish system, maybe, but it's pretty complicated.
//! 
//! The issue is not just with the order at which each visitor is present, but also matter within each visitor.

//Note: we're trying to avoid that dreaded &dyn. it's stinky, silly, annoying, and hard to work with. 

use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;

use chunks::{ReadChunk, WriteChunk};
use core_obj::*;
use pos::ChunkPos;
use prelude::*;

pub mod chunks;
pub mod pos;

const CHUNK_SIZE: u8 = 32;
const CHUNK_SIZE_I32: i32 = 32;
const CHUNK_SIZE_I8: i8 = 32;

#[async_trait]
pub trait Map<R>
where
    R: Registrar,
    Self: Sized,
{
    type EventListener: EventListener<R>;
    /// Read/write to existing file or make a new one
    async fn init(path: Arc<Path>, runtime: &R) -> Result<(Self, Self::EventListener)>;
    /// externally push an event.
    async fn push_event(&self, alert: VoxEvent<R>) -> Result<()>;
    type ReadChunk<'a>: ReadChunk<R> where Self: 'a;
    type WriteChunk<'a>: WriteChunk<R> where Self: 'a;

    async fn read_chunk<'b>(&'b self, pos: ChunkPos) -> Option<Self::ReadChunk<'b>>;
    async fn write_chunk<'b>(&'b mut self, pos: ChunkPos) -> Option<Self::WriteChunk<'b>>;
}

///A struct that you can be to notified if:
/// - Something happens at a specific chunk
/// - Something happens regarding a specific voxel type
/// - Whenever some specific event happens
pub trait EventListener<R: Registrar>{
    //TODO: how do we want this to work? (decide once we need it)
}
/// A notification waking up a voxel.
#[derive(Debug, Clone, Copy)]
pub struct VoxEvent<R: Registrar>{
    pub pos: Pos,
    pub vox_type: R::VoxelType,
    pub event_type: EventType<R>
}
#[derive(Debug, Clone, Copy)]
pub enum EventType<R: Registrar>{
    Type(R::VoxelType),
    Neighbor,
    Removed,
    Inventory,
}