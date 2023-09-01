use std::{collections::VecDeque, mem};
use crate::{
    obj::{
        dim::{self, MapError, DimTypeInstancePtr},
        element::Element,
        voxel::Voxel,
    },
    pos::{GlobalPos, RelativePos, Pos},
    Seed, display::map::{ListenableMap, MapListener, MapUpdate},
};
use async_trait::async_trait;
use sc_prelude::*;
use self::{chunk::Chunk, octree::Octree};
use crate::CHUNK_SIZE;
use load::LoadOpt;

mod chunk;
mod load;
mod octree;

#[derive(Debug)]
pub struct Map<V, E>
where
    V: Debug + Clone + Send,
    E: Debug,
{
    chunks: Octree<LoadOpt<Chunk<Option<V>, CHUNK_SIZE>>>,
    seed: Seed,
    to_generate: VecDeque<GlobalPos>,
    to_load: VecDeque<GlobalPos>,
    update_tx: Option<flume::Sender<MapUpdate>>,
    _e: PhantomData<E>,
}
#[async_trait]
impl dim::MapTrait for Map<Voxel, Element> {
    fn new() -> Self {
        Map::default()
    }

    fn set_seed(&mut self, seed: Seed) {
        self.seed = seed;
    }

    fn get(&self, pos: GlobalPos) -> Result<&Option<Voxel>, MapError> {
        let Some(chunk) = self.chunks.get_weak(pos.try_tuple().unwrap()) else {
            return Err(MapError::UnGenerated);
        };
        match chunk{
            LoadOpt::Loaded(chunk) => Ok(chunk.get(pos.relative())),
            LoadOpt::Unloaded(_) => Err(MapError::Unloaded),
        }

    }

    fn get_mut_weak(&mut self, pos: GlobalPos) -> Result<&mut Option<Voxel>, MapError> {
        match self.chunks.get_mut_weak(pos.try_tuple().unwrap()){
            Some(LoadOpt::Loaded(chunk)) => Ok(chunk.get_mut(pos.relative())),
            Some(LoadOpt::Unloaded(_)) => {
                Err(MapError::Unloaded)
            },
            None => {
                Err(MapError::UnGenerated)
            }
        }
    }

    fn get_mut_strong(&mut self, pos: GlobalPos) -> Result<&mut Option<Voxel>, MapError> {
        match self.chunks.get_mut_weak(pos.try_tuple().unwrap()){
            Some(LoadOpt::Loaded(chunk)) => Ok(chunk.get_mut(pos.relative())),
            Some(LoadOpt::Unloaded(_)) => {
                self.to_load.push_back(pos);
                Err(MapError::Unloaded)
            },
            None => {
                self.to_generate.push_back(pos);
                Err(MapError::UnGenerated)
            }
        }
    }

    async fn load(&mut self) -> Result<()> {
        for pos in self.to_load.drain(..){
            if let Some(chunk) = self.chunks.get_mut_weak(pos.chunk()){
                chunk.try_load().await?
            }
            else{
                //TODO: log
                self.to_generate.push_back(pos);
            }
        }
        Ok(())
    }

    fn gen(&mut self, dim: &DimTypeInstancePtr) -> Result<()> {
        for pos in mem::take(&mut self.to_generate){
            self.generate_chunk(dim, pos)?;
        }
        Ok(())
    }

}
impl<V, E> Map<V, E>
where
    V: Debug + Clone + Send,
    E: Debug,
{

}
impl<V, E> Default for Map<V, E>
where
    V: Debug + Clone + Send,
    E: Debug,
{
    fn default() -> Self {
        Self { 
            chunks: Default::default(), 
            seed: Default::default(), 
            to_generate: Default::default(), 
            to_load: Default::default(), 
            update_tx: None,
            _e: Default::default(), 
        }
    }
}
impl<E: Debug> Map<Voxel,E>{
    fn generate_chunk(&mut self, dim: &DimTypeInstancePtr, pos: GlobalPos) -> Result<()>{
        let global = pos;
        let mut new: Chunk<Option<Voxel>, CHUNK_SIZE> = Chunk::default();
        for relative in Chunk::<Option<Voxel>, CHUNK_SIZE>::all_pos(){
            let generate_position = GlobalPos::new_from_parts(global.chunk(), *relative);
            *new.get_mut(*relative) = dim.as_ref().gen(self.seed, generate_position);
        }
        *self.chunks.get_mut_strong(pos.chunk()) = LoadOpt::new(new);        
        Ok(())
    }
}
impl<V, E> ListenableMap for Map<V, E>
where
    V: Debug + Clone + Send,
    E: Debug,
{
    fn new_rx(&mut self) -> Self::Listener {
        let (tx, rx) = flume::unbounded();
        self.update_tx = Some(tx);
        Listener(rx)
    }

    type Listener = Listener;
}

pub struct Listener(flume::Receiver<MapUpdate>);
impl MapListener for Listener{
    fn block_rx(&self) -> MapUpdate {
        todo!()
    }

    fn try_rx(&self) -> Option<MapUpdate> {
        todo!()
    }
}
 
//TRAITS of dependency inversion...
trait OctreeTrait<T: Default + Debug>: Debug + Default {
    fn new(size: u16) -> Self;
    fn get_weak(&self, pos: (i16, i16, i16)) -> Option<&T>;
    /// Will not create a new one if this position doesn't exist.
    fn get_mut_weak(&mut self, pos: (i16, i16, i16)) -> Option<&mut T>;
    /// Will create a new one if this position doesn't exist.
    fn get_mut_strong(&mut self, pos: (i16, i16, i16)) -> &mut T;
}

trait ChunkTrait<T: Default + Debug + Clone + Sized>: Debug + Default {
    fn new() -> Self;
    fn get(&self, pos: RelativePos) -> &T;
    fn get_mut(&mut self, pos: RelativePos) -> &mut T;
    fn all_pos() -> &'static Vec<RelativePos>;
}
