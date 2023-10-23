use std::collections::BinaryHeap;

use core_obj::{Data, Pos, TypeId, Voxel};
use db_protocol::visit::{Message, self, VoxelVisit};
use prelude::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::CHUNK_SIZE;
use crate::{ChunkTrait, CwPos, CHUNK_SIZE_I32};

#[derive(Debug)]
pub(crate) struct Chunk<T: TypeId, D: Data, M: Message> {
    voxels: [[[Option<Voxel<T, D>>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    messages: (flume::Sender<(Pos,M)>, flume::Receiver<(Pos,M)>),
    __messages_heap: BinaryHeap<PosMsg<M>>,
    __messages_locations: [[[(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    contained_attr: Vec<T::AttrId>, //TODO: does not actually update atm, just sits there
}
impl<T: TypeId, D: Data, M: Message> ChunkTrait<T, D, M> for Chunk<T, D, M> {
    fn contains_attr(&self, attr: T::AttrId) -> bool {
        self.contained_attr.contains(&attr)
    }

    fn tell(&self, pos: Pos, msg: M) {
        self.messages.0.send((pos,msg)).unwrap()
    }

    fn get(&self, pos: Pos) -> &Option<Voxel<T, D>> {
        &self.voxels[pos.x as usize][pos.y as usize][pos.z as usize]
    }
    fn get_mut(&mut self, pos: Pos) -> &mut Option<Voxel<T, D>> {
        &mut self.voxels[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    fn get_visited<V>(&self, map: &crate::Map<T,D,M>, cw_pos: CwPos, visitor: &V) 
    where V: visit::VoxelVisitor<T,D, M, crate::Map<T,D,M>> + Send + Sync {
        ALL_POS.par_iter()
        .filter_map(|pos|{
            match self.get(*pos){
                Some(vox) => Some((pos,vox)),
                _ => None,
            }
        }).for_each(|(pos,voxel)|{
            
            todo!()
        });
    }

}
impl<T: TypeId, D: Data, M: Message> Default for Chunk<T, D, M> {
    fn default() -> Self {
        let arr_1d: [_; CHUNK_SIZE] = std::array::from_fn(|_| None);
        let arr_2d: [[_; CHUNK_SIZE]; CHUNK_SIZE] = std::array::from_fn(|_| arr_1d.clone());
        let arr_3d: [[[_; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] =
            std::array::from_fn(|_| arr_2d.clone());

        Self {
            voxels: arr_3d,
            messages: flume::unbounded(),
            contained_attr: Vec::new(),
            __messages_heap: BinaryHeap::new(),
            __messages_locations: todo!(),
        }
    }
}

lazy_static! {
    static ref ALL_POS: Vec<Pos> = {
        fn all_pos<const S: i32>() -> impl Iterator<Item = Pos> {
            (0..S).flat_map(|x| (0..S).flat_map(move |y| (0..S).map(move |z| Pos::new(x, y, z))))
        }
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


#[derive(Debug)]
pub struct PosMsg<M: Message + Debug>(Pos, M);
impl<M: Message> Ord for PosMsg<M>{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match self.0.x.cmp(&other.0.x){
            Equal => match self.0.y.cmp(&other.0.y){
                Equal => self.0.z.cmp(&other.0.z),
                ord => ord,
            },
            ord => ord,
        }
    }
}
impl<M: Message> PartialOrd for PosMsg<M>{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<M: Message> Eq for PosMsg<M>{
    
}
impl<M: Message> PartialEq for PosMsg<M>{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod test {
    
}
