#![feature(slice_flatten)]

use std::path::Path;

use async_trait::async_trait;
use rustc_hash::FxHashMap;

use core_obj::Runtime;
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
R: Runtime, 
{
    chunks: FxHashMap<ChunkPos, chunk::Chunk<R>>
}
#[async_trait]
impl<R> world_protocol::Map<R> for Map<R> 
where 
R: Runtime + Sync + Send, 
{
    ///Read/write to existing file or make a new one
    async fn init(path: &Path, runtime: &R) -> Result<Self>{
        todo!()
    }  
    async fn add_event(&self, alert: VoxEvent<R>) -> Result<Self>{
        todo!()
    }  
    fn get_events(&self) -> &[VoxEvent<R>]{
        todo!()
    }  
    async fn clear_events(&mut self){
        todo!()
    }

    
    type ReadChunk<'a> = chunk::ReadChunk<'a, R> where Self: 'a;
    type WriteChunk<'a> = chunk::WriteChunk<'a, R> where Self: 'a;

    async fn write_chunk<'b>(&'b self, pos: ChunkPos) -> Option<Self::ReadChunk<'b>>{
        todo!()
    }
    async fn read_chunk<'b>(&'b mut self, pos: ChunkPos) -> Self::WriteChunk<'b>{
        todo!()
    }
}
impl<R> Default for Map<R> 
where 
R: Runtime, 
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


