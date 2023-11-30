

use core_obj::{Type, Data, Pos, Voxel};
use world_protocol::{message::VoxelMsg};

use crate::{CHUNK_SIZE, LocalPos};

#[derive(Debug, Clone)]
pub struct Chunk<Vox: Voxel + Send + Sync>{
    voxels: [[[Option<Vox>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    contains_attr: Vec<  <Vox as Voxel>::AttrType  >, // a mess bro
    messages: (flume::Sender<(LocalPos,VoxelMsg<Vox>)>, flume::Receiver<(LocalPos,VoxelMsg<Vox>)>),
}

impl<Vox: Voxel + Send + Sync> world_protocol::Chunk<Vox> for Chunk<Vox>{
    fn msg(&self, pos: Pos, msg: VoxelMsg<Vox>) {
        todo!()
    }

    fn get_pos(&self, pos: Pos) -> world_protocol::BoundsResult<&Option<Vox>> {
        todo!()
    }

    fn get_pos_mut(&mut self, pos: Pos) -> world_protocol::BoundsResult<&mut Option<Vox>> {
        todo!()
    }

    fn read_phase<M, V>(&self, visitor: V) 
    where M: world_protocol::Map<Vox>, V: world_protocol::VisitorRead<Vox, M> {
        todo!()
    }

    fn respond_phase<M, V>(&mut self, visitor: V) 
    where M: world_protocol::Map<Vox>, V: world_protocol::VisitorRespond<Vox, M> {
        todo!()
    }

    fn apply_phase<M, V>(&mut self, visitor: V) 
    where M: world_protocol::Map<Vox>, V: world_protocol::VisitorApply<Vox, M> {
        todo!()
    }
}


impl<Vox: Voxel + Send + Sync> super::ChunkTrait<Vox> for Chunk<Vox>{
    fn get_type(&self, pos: LocalPos) -> Option<<Vox as Voxel>::Type> {
        todo!()
    }

    fn tell(&self, pos: LocalPos, msg: VoxelMsg<Vox>) {
        todo!()
    }
}


impl<V: Voxel + Send + Sync> Default for Chunk<V>{
    fn default() -> Self {
        prelude::lazy_static! {
            static ref ALL_POS: Vec<Pos> = {
                let size = CHUNK_SIZE as i32;
                let mut vec: Vec<_> = Vec::with_capacity(CHUNK_SIZE.pow(3) as usize);
                for x in 0..size {
                    for y in 0..size {
                        for z in 0..size {
                            vec.push(Pos::new(x, y, z));
                        }
                    }
                }
                vec
            };
        }
        let arr_1d: [_; CHUNK_SIZE] = std::array::from_fn(|_| None);
        let arr_2d: [[_; CHUNK_SIZE]; CHUNK_SIZE] = std::array::from_fn(|_| arr_1d.clone());
        let arr_3d: [[[_; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] =
            std::array::from_fn(|_| arr_2d.clone());

        Self {
            voxels: arr_3d,
            contains_attr: Vec::new(),
            messages: flume::unbounded(),
        }
    }
}

