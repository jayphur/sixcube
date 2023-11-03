use std::marker::PhantomData;

use core_obj::{Type, Data, Pos, Voxel};
use world_protocol::visit::{Message, VoxelVisitor};

use crate::{ChunkTrait, CHUNK_SIZE, LocalPos, message::Msg};

#[derive(Debug, Clone)]
pub struct Chunk<V: Voxel + Send + Sync>{
    voxels: [[[Option<V>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    contains_attr: Vec<  <V as Voxel>::AttrType  >, // a mess bro
    messages: (flume::Sender<(LocalPos,Msg)>, flume::Receiver<(LocalPos,Msg)>),
}

impl<Vox: Voxel + Send + Sync> ChunkTrait<Vox> for Chunk<Vox>{
    fn contains_attr(&self, attr_id: &Vox::AttrType) -> bool {
        self.contains_attr.contains(attr_id)   
    }

    fn get_type(&self, pos: LocalPos) -> Option<Vox::Type> {
        Some(*self.voxels[pos.z][pos.y][pos.x].as_ref()?.get_type())
    }

    fn tell(&self, pos: LocalPos, msg: Msg) {
        self.messages.0.send((pos,msg)).unwrap();
    }

    fn get_visited<V>(&self, map: &crate::Map<Vox>, pos: Pos, visitor: &V)
    where V: Send + Sync + world_protocol::visit::VoxelVisitor<Vox, crate::Map<Vox>> {
        todo!()
    }

    fn get_visited_mut<V>(&mut self, pos: Pos, visitor: &V)
    where V: Send + Sync + world_protocol::visit::VoxelVisitor<Vox, crate::Map<Vox>>  {
        todo!()
    }
}

impl<V: Voxel + Send + Sync> Default for Chunk<V>{
    fn default() -> Self {
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