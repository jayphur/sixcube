use std::{marker::PhantomData, ops::DerefMut, iter, mem};

use core_obj::{Pos, Runtime, Voxel};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use world_protocol::{Visitor, VisitorRegistry};
use prelude::*;
use crate::{CHUNK_SIZE, Pos16, Map, CHUNK_SIZE_I32, LocalPos, PosU};

#[derive(Debug)]
pub(crate) struct Chunk<R> 
where 
R: Runtime, 
{
    pub cw_pos: Pos16,
    pub data: RwLock<ChunkData<R>>,
    pub msg_tx: tokio::sync::mpsc::Sender<ChunkMsg>
}
#[derive(Debug)]

pub struct ChunkData<R:Runtime>{ //TODO: special case for empty chunk?
    msg_rx: tokio::sync::mpsc::Receiver<ChunkMsg>,
    voxels: Arr3d<Option<StoredVoxel<R>>>,
    vox_data: FxHashMap<LocalPos, ()>, //TODO: () is the data stored in the voxel
    visitors_list: Vec<u16>, //TODO: Nothing fills this atm
}

/// Lighter/decentralized version of a voxel.
#[derive(Debug, Default, Clone, Copy)]
struct StoredVoxel<R:Runtime>{
        type_id: R::VoxelType,
        has_data: bool,
    
}

impl<R: Runtime> PartialEq for StoredVoxel<R> {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl<R> Chunk<R> 
where 
R: Runtime + Sync + Send, 
{
    pub async fn update<'a, 'v, V: VisitorRegistry<'v, R, Map<R>>>(&self, registry: &V, map: &Map<R>){
        let mut data = self.data.write().await;
        for visitor in registry.get_visitor(data.visitors_list.as_slice()){
            let pos16 = self.cw_pos;
            let origin = (pos16.0 as i32 *CHUNK_SIZE_I32,pos16.1 as i32 *CHUNK_SIZE_I32,pos16.2 as i32 *CHUNK_SIZE_I32);
            for pos_usize in all_pos::<usize>(){
                let pos = Pos::from_usize(pos_usize);
                let pos = Pos(origin.0 + pos.0,origin.1 + pos.1,origin.2 + pos.2);
                let Some(stored_vox) = data.voxels.get_mut(pos_usize.into()) else {continue;};
                let voxel_mut: VoxelMut<'_,R> = VoxelMut{
                    my_type: &mut stored_vox.type_id,
                };
                visitor.visit(pos, voxel_mut);
            }
        }
    }

    pub fn new(cw_pos: Pos) -> Self {
        todo!()
    }

    pub fn get_type(&self, pos: crate::LocalPos) -> Option<R::VoxelType> {
        todo!()
    }

}

struct VoxelMut<'a, R: Runtime>{
    my_type: &'a mut R::VoxelType,
}
impl<'a, R: Runtime> world_protocol::VoxelMut<'a, R> for VoxelMut<'a,R>{
    #[inline(always)]
    fn get_my_type(&self) -> &<R as Runtime>::VoxelType { &self.my_type }
    #[inline(always)]
    fn set_my_type(&mut self, val: <R as Runtime>::VoxelType) { *self.my_type = val }
}

#[derive(Debug)]
pub enum ChunkMsg{

}

fn all_pos<T>() -> impl Iterator<Item=(T,T,T)>
where
T: From<usize>
{
    (0..CHUNK_SIZE).into_iter()
        .flat_map(|x|{
            (0..CHUNK_SIZE)
                .into_iter()
                .flat_map(move |y|{
                    (0..CHUNK_SIZE)
                        .into_iter()
                        .map(move |z|{
                            (T::from(x),T::from(y),T::from(z))
                        })
                })
        })
}

pub struct ChunkBuilder<R:Runtime>{
    __marker: PhantomData<R>,
}
impl<R:Runtime> ChunkBuilder<R>{

}


///W will have to be n^3, sorry.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Arr3d<T>(pub(crate )[[[T;CHUNK_SIZE];CHUNK_SIZE];CHUNK_SIZE]) where T: Clone + Debug + Default + PartialEq;

impl<T> Arr3d<T>
where T: Clone + Debug + Default + PartialEq
{
    pub fn get(&self, pos: PosU) -> &T{
        &self.0[pos.0][pos.1][pos.2]
    }
    pub fn get_mut(&mut self, pos: PosU) -> &mut T{
        &mut self.0[pos.0][pos.1][pos.2]
    }
}