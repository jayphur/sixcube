use std::{marker::PhantomData, iter};

use core_obj::{Type, Data, Pos};
use world_protocol::visit::VoxelVisitor;
use message::Msg;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, ParallelBridge, IntoParallelIterator};
use spatialtree::{OctTree, OctVec, Tree};
use chunk::Chunk;

mod chunk;
mod message;


// Tree tree tree tree tree!!!!!

//Smallest chunk 32^3
//nah maybe 8^3

type LocalPos = vector3d::Vector3d<usize>;

const CHUNK_SIZE: usize = 16;
const CHUNK_SIZE_I32: i32 = 16;

#[derive(Debug, Clone)]
pub struct Map<V: core_obj::Voxel + Send + Sync>{
    loaded_chunks: OctTree<Chunk<V>, OctVec<u32>>,
    all_chunks: OctTree<(), OctVec<u32>>,
}
impl<Vox: core_obj::Voxel + Send  + Sync> world_protocol::Map<Vox> for Map<Vox> 
{
    fn get_type(&self, pos: Pos) -> Option<Vox::Type> {
        if let Some(chunk) = self.loaded_chunks.get_chunk_by_position(pos_to_oct_pos(pos)){
            Some(chunk.get_type(pos_to_local_pos(pos))?) 
        } else {
            let chunk = self.all_chunks.get_chunk_by_position(pos_to_oct_pos(pos))?;
            todo!()
        }
    }

    fn tell(&self, pos: Pos, msg: Msg) {
        if let Some(chunk) = self.loaded_chunks.get_chunk_by_position(pos_to_oct_pos(pos)){
            chunk.tell(pos_to_local_pos(pos), msg)
        }
    }

    fn message_phase<'v, V>(&mut self, visitors: &'v [V])
    where V: 'v + Send + Sync + world_protocol::visit::VoxelVisitor<Vox, Self> {
        (0..self.loaded_chunks.get_num_chunks())
        .into_par_iter()
        .map(|index|{
            self.loaded_chunks.get_chunk(index)
        })
        .map(|container|{
            let pos = container.position();
            let chunk = &container.chunk;
            (
                oct_pos_to_pos(pos),
                chunk
            )
        }).for_each(|(pos, chunk)|{
            visitors.par_iter().filter(|visitor|{
                visitor.predicate().with_attributes.iter().any(|a|chunk.contains_attr(a))
            })
            .for_each(|visitor|{
                chunk.get_visited(self, pos, visitor)
            });
        })
    }

    fn respond_phase<'v, V>(&mut self, visitors: &'v [V])
    where V: 'v + Send + Sync + world_protocol::visit::VoxelVisitor<Vox, Self> {
        self.loaded_chunks
            .iter_chunks_mut()
            .par_bridge()
            .map(|(_, chunk_container)|{
                (
                    oct_pos_to_pos(chunk_container.position()),
                    &mut chunk_container.chunk
                )
            }).for_each(|(pos, chunk)|{
                for visitor in visitors.iter(){ // It must be sequential because it's &mut
                    if visitor
                        .predicate()
                        .with_attributes
                        .iter()
                        .any(|a|chunk.contains_attr(a))
                    {
                        continue;
                    } else{
                        chunk.get_visited_mut(pos, visitor)
                    }
                }
            })
    }

    type Msg = Msg;
}

impl<Vox: core_obj::Voxel + Send + Sync> Default for Map<Vox>{
    fn default() -> Self {
        Self { 
            loaded_chunks: OctTree::<Chunk<Vox>, OctVec<u32>>::new(),
            all_chunks:  OctTree::<(), OctVec<u32>>::new(),
        }
    }
}


const U32_MIDPOINT: u32 = 32768;

pub fn pos_to_oct_pos(pos: Pos) -> OctVec<u32>{
    OctVec::new([
        i32_to_u32(pos.x),
        i32_to_u32(pos.y),
        i32_to_u32(pos.z),
    ], 0)
}
pub fn oct_pos_to_pos(pos: OctVec<u32>) -> Pos{
    Pos::new(u32_to_i32(pos.pos[0]), u32_to_i32(pos.pos[1]), u32_to_i32(pos.pos[2]))
}
pub fn i32_to_u32(val: i32) -> u32{
    if val.is_negative(){
        val.abs() as u32 
    } else {
        val as u32 + U32_MIDPOINT
    }
}
pub fn pos_to_local_pos(pos: Pos) -> LocalPos{
    LocalPos::new((pos.x%CHUNK_SIZE_I32) as usize, (pos.y%CHUNK_SIZE_I32) as usize, (pos.z%CHUNK_SIZE_I32) as usize)
}

pub fn u32_to_i32(val: u32) -> i32{
    use std::cmp::Ordering;
    match val.cmp(&U32_MIDPOINT){
        Ordering::Less => {
            -(val as i32)
        },
        Ordering::Equal => {
            0
        },
        Ordering::Greater => {
            (val - U32_MIDPOINT) as i32
        },
    }
}

/// DEPENDENCY INVERSION
pub trait ChunkTrait<Vox: core_obj::Voxel + Send + Sync>{
    fn contains_attr(&self, attr_id: &Vox::AttrType) -> bool;    
    fn get_visited<V>(&self, map: &Map<Vox>, pos: Pos, visitor: &V)
    where V: Send + Sync + world_protocol::visit::VoxelVisitor<Vox, Map<Vox>>;
    fn get_visited_mut<V>(&mut self, pos: Pos, visitor: &V)
    where V: Send + Sync + world_protocol::visit::VoxelVisitor<Vox, Map<Vox>> ;
    fn get_type(&self, pos: LocalPos) -> Option<Vox::Type>;
    fn tell(&self, pos: LocalPos, msg: Msg); 
}

#[cfg(test)]
mod test_conv{

    #[test]
    fn there_and_back(){
        let start: i32 = 31785;
        assert_eq!(start, super::u32_to_i32(super::i32_to_u32(start)));

        let start: i32 = -3185;
        assert_eq!(start, super::u32_to_i32(super::i32_to_u32(start)));

        let start: i32 = -3;
        assert_eq!(start, super::u32_to_i32(super::i32_to_u32(start)));
        
        let start: i32 = 0;
        assert_eq!(start, super::u32_to_i32(super::i32_to_u32(start)));
    }
}