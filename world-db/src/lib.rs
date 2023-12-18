#![feature(slice_flatten)]
use std::{path::Path, io::BufWriter, marker::PhantomData, sync::Arc, mem::MaybeUninit};
use async_trait::async_trait;
use core_obj::{Pos, Runtime};
use prelude::*;
use rustc_hash::FxHashMap;
use tokio::sync::{mpsc, broadcast};
use world_protocol::{VisitorRegistry, Update};
use chunk::{Chunk, ChunkData};

mod chunk;
mod disk;
mod bg_tree;
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
    active_chunks: FxHashMap<Pos16, Chunk<R>>,
    //bg_chunks: bg_tree::BgTree<R::VoxelType>,
    update_tx: broadcast::Sender<Update<R>>,
}
#[async_trait]
impl<R> world_protocol::Map<R> for Map<R> 
where 
R: Runtime + Sync + Send, 
{
    type UpdateListener = UpdateListener<R>;
    async fn init(path: &Path, runtime: &R) -> Result<Self>{
        todo!()
    }
    async fn get_type(&self, pos: Pos, runtime: &R) -> Option<R::VoxelType>{
        todo!()
    }
    async fn update<'v, V>(&mut self, registry: &V, runtime: &R) where V: VisitorRegistry<'v, R, Self>{
        todo!()
    }
    fn new_listener(&self) -> Self::UpdateListener{
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


#[derive(Debug)]
pub struct UpdateListener<R>
where
R: Runtime + Sync, 
{
    rx: broadcast::Receiver<Update<R>>,
    __marker: PhantomData<R>
}
#[async_trait]
impl<R: Runtime + Sync + Send> world_protocol::UpdateListener<R> for UpdateListener<R> {
    
    async fn rx_async(&mut self) -> Result<Update<R>>{
        Ok(self.rx.recv().await?)
    }
    fn try_rx(&mut self) -> Result<Option<Update<R>>>{
        Ok(Some(self.rx.try_recv()?))
    }
    fn rx_blocking(&mut self) -> Result<Update<R>>{
        Ok(self.rx.blocking_recv()?)
    }
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(crate) struct Pos16(pub i16,pub i16,pub i16); 
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct LocalPos(pub u8,pub u8,pub u8);
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(crate) struct PosU(usize,usize,usize);

impl From<(usize,usize,usize)> for PosU {
    fn from(value: (usize,usize,usize)) -> Self {
        Self(value.0,value.1,value.2)
    }
}


/// Turn a global position into chunk wise + local 
pub(crate) fn break_down(pos: Pos) -> (Pos16, LocalPos){
    todo!()
}