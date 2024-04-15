#![feature(slice_flatten)]
#![feature(result_flattening)]
#![feature(array_try_from_fn)]


use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use rustc_hash::FxHashMap;

use core_obj::Registrar;
use prelude::*;
use world_protocol::{pos::{ChunkLocalPos, ChunkPos}, VoxEvent};

use crate::disk::MapFile;

mod arr3d;
mod disk;
mod chunk;

// Tree tree tree tree tree!!!!!
//Smallest chunk 32^3
//nah maybe 8^3

const CHUNK_SIZE: usize = 32;
const CHUNK_SIZE_I32: i32 = 32;


#[derive(Debug)]
pub struct Map {
    chunks: FxHashMap<ChunkPos, chunk::Chunk>,
    file: MapFile<Arc<Path>>,

}
#[async_trait]
impl world_protocol::Map for Map{
    type EventListener = EventListener;

    ///Read/write to existing file or make a new one
    async fn init(path: Arc<Path>, registrar: &Registrar) -> Result<(Self, Self::EventListener)>{
        let map_file = MapFile::init(path, registrar).await?;
        let map = Self{
            chunks: Default::default(),
            file: map_file,
        };
        let listener = EventListener{

        };
        Ok((map, listener))
    }
    async fn push_event(&self, alert: VoxEvent) -> Result<()>{
        todo!()
    }


    type ReadChunk<'a> = chunk::ReadChunk<'a> where Self: 'a;
    type WriteChunk<'a> = chunk::WriteChunk<'a> where Self: 'a;

    async fn read_chunk<'b>(&'b self, pos: ChunkPos) -> Option<Self::ReadChunk<'b>>{
        Some(self.chunks.get(&pos)?.read(pos).await)
    }
    async fn write_chunk<'b>(&'b mut self, pos: ChunkPos) -> Option<Self::WriteChunk<'b>>{
        Some(self.chunks.get(&pos)?.write(pos).await)
    }

}
impl Default for Map{
    fn default() -> Self {
        todo!()
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct PosU(usize, usize, usize);

impl PosU {
    #[inline]
    fn tuple(&self) -> (usize,usize,usize){
        (self.0,self.1,self.2)
    }
}

impl From<ChunkLocalPos> for PosU {
    fn from(value: ChunkLocalPos) -> Self {
        Self(value.0 as usize, value.1 as usize, value.2 as usize)
    }
}

#[derive(Debug)]
pub struct EventListener{

}
impl world_protocol::EventListener for EventListener {

}