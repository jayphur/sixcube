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
pub struct Map<R> 
where 
R: Registrar,
{
    chunks: FxHashMap<ChunkPos, chunk::Chunk<R>>,
    file: MapFile<R, Arc<Path>>,

}
#[async_trait]
impl<R> world_protocol::Map<R> for Map<R> 
where 
R: Registrar + Sync + Send + 'static,
{
    type EventListener = EventListener<R>;

    ///Read/write to existing file or make a new one
    async fn init(path: Arc<Path>, registrar: &R) -> Result<(Self, Self::EventListener)>{
        let map_file = MapFile::init(path, registrar).await?;
        let map = Self{
            chunks: Default::default(),
            file: map_file,
        };
        let listener = EventListener{

        };
        Ok((map, listener))
    }
    async fn push_event(&self, alert: VoxEvent<R>) -> Result<()>{
        todo!()
    }


    type ReadChunk<'a> = chunk::ReadChunk<'a, R> where Self: 'a;
    type WriteChunk<'a> = chunk::WriteChunk<'a, R> where Self: 'a;

    async fn read_chunk<'b>(&'b self, pos: ChunkPos) -> Option<Self::ReadChunk<'b>>{
        Some(self.chunks.get(&pos)?.read(pos).await)
    }
    async fn write_chunk<'b>(&'b mut self, pos: ChunkPos) -> Option<Self::WriteChunk<'b>>{
        Some(self.chunks.get(&pos)?.write(pos).await)
    }

}
impl<R> Default for Map<R> 
where 
R: Registrar,
{
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
pub struct EventListener<R: Registrar>{

}
impl<R: Registrar> world_protocol::EventListener<R> for EventListener<R> {

}