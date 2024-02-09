#![feature(slice_flatten)]

use std::ops::Deref;
use std::path::Path;

use async_trait::async_trait;
use rustc_hash::FxHashMap;
use tokio::sync::mpsc;

use core_obj::Registrar;
use prelude::*;
use world_protocol::{pos::{ChunkLocalPos, ChunkPos}, VoxEvent};

mod arr3d;
mod disk;
mod chunk;

// Tree tree tree tree tree!!!!!
//Smallest chunk 32^3
//nah maybe 8^3

const CHUNK_SIZE: usize = 16;
const CHUNK_SIZE_I32: i32 = 16;


#[derive(Debug)]
pub struct Map<R> 
where 
R: Registrar,
{
    chunks: FxHashMap<ChunkPos, chunk::Chunk<R>>,
    events: mpsc::Sender<VoxEvent<R>>,
}
#[async_trait]
impl<R> world_protocol::Map<R> for Map<R> 
where 
R: Registrar + Sync + Send + 'static,
{
    ///Read/write to existing file or make a new one
    async fn init(path: &Path, runtime: &R) -> Result<(Self, std::sync::mpsc::Receiver<VoxEvent<R>>)>{
        todo!()
    }  
    async fn add_event(&self, alert: VoxEvent<R>) -> Result<()>{
        self.events.send(alert).await?;
        Ok(())
    }
    
    type ReadChunk<'a> = chunk::ReadChunk<'a, R> where Self: 'a;
    type WriteChunk<'a> = chunk::WriteChunk<'a, R> where Self: 'a;

    async fn read_chunk<'b>(&'b self, pos: ChunkPos) -> Option<Self::ReadChunk<'b>>{
        Some(self.chunks.get(&pos)?.read(pos).await)
    }
    async fn write_chunk<'b>(&'b mut self, pos: ChunkPos) -> Option<Self::WriteChunk<'b>>{
        Some(self.chunks.get(&pos)?.write(pos).await)
    }

    fn init_chunk(&mut self, pos: ChunkPos) {
        todo!()
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

impl From<ChunkLocalPos> for PosU {
    fn from(value: ChunkLocalPos) -> Self {
        Self(value.0 as usize, value.1 as usize, value.2 as usize)
    }
}


