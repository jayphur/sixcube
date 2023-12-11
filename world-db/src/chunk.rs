

use std::{marker::PhantomData, default};

use core_obj::{Pos, Voxel, Runtime};
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use world_protocol::{message::VoxelMsg, Visitor, VisitorRegistry};

use crate::{CHUNK_SIZE, Pos16};

#[derive(Debug)]
pub(crate) struct Chunk<R> 
where 
R: Runtime, 
{
    pub cw_pos: Pos16,
    pub data: RwLock<ChunkData<R>>,
    pub msg_tx: flume::Sender<ChunkMsg>
}
#[derive(Debug, Clone)]

pub struct ChunkData<R:Runtime>{ //TODO: special case for empty chunk?
    msg_rx: flume::Receiver<ChunkMsg>,
    voxels: [[[StoredVoxel<R>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    data: FxHashMap<LocalPos, ()> //TODO: () is the data stored in the voxel
}
#[derive(Debug, Default, Clone, Copy)]
enum StoredVoxel<R:Runtime>{
    #[default]
    None,
    Some{
        type_id: R::VoxelType,
        has_data: bool,
    }
}

impl<R> Chunk<R> 
where 
R: Runtime, 
{
    fn read_phase<'a, V>(&self, registry: &V, map: &crate::Map<R>) 
    where V: VisitorRegistry<'a, R, crate::Map<R>> 
    {
        todo!()
    }

    fn respond_phase<'a, V>(&self, registry: &V) 
    where V: VisitorRegistry<'a, R, crate::Map<R>> 
    {
        todo!()
    }

    fn apply_phase<'a, V>(&self, registry: &V) 
    where V: VisitorRegistry<'a, R, crate::Map<R>> 
    {
        todo!()
    }

    fn new(cw_pos: Pos) -> Self {
        todo!()
    }

    fn get_type(&self, pos: crate::LocalPos) -> Option<R::VoxelType> {
        todo!()
    }

    fn tell(&self, pos: crate::LocalPos, msg: VoxelMsg<R>) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalPos{

}

#[derive(Debug)]
pub enum ChunkMsg{

}