use std::{marker::PhantomData, iter};

use conversion::pos_to_local_pos;
use core_obj::{Type, Data, Pos, Voxel};
use world_protocol::{message::VoxelMsg, VisitorRead, VisitorRespond, VisitorApply, VisitingPredicate};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, ParallelBridge, IntoParallelIterator};
use spatialtree::{OctTree, OctVec, Tree};
use chunk::Chunk;

use crate::conversion::pos_to_oct_pos;

mod chunk;


// Tree tree tree tree tree!!!!!

//Smallest chunk 32^3
//nah maybe 8^3

mod conversion;
type LocalPos = vector3d::Vector3d<usize>;

const CHUNK_SIZE: usize = 16;
const CHUNK_SIZE_I32: i32 = 16;

#[derive(Debug, Clone)]
pub struct Map<V: core_obj::Voxel + Send + Sync>{
    loaded_chunks: OctTree<Chunk<V>, OctVec<u32>>,
    all_chunks: OctTree<(), OctVec<u32>>, //TODO: actually store "all chunks" not just ()
}
impl<Vox: core_obj::Voxel + Send  + Sync> world_protocol::Map<Vox> for Map<Vox> 
{
    type Chunk = Chunk<Vox>;

    fn get_type(&self, pos: Pos) -> Option<Vox::Type> {
        if let Some(chunk) = self.loaded_chunks.get_chunk_by_position(pos_to_oct_pos(pos)){
            Some(chunk.get_type(pos_to_local_pos(pos))?) 
        } else {
            let chunk = self.all_chunks.get_chunk_by_position(pos_to_oct_pos(pos))?;
            todo!()
        }
    }

    fn msg_voxel(&self, pos: Pos, msg: VoxelMsg<Vox>) {
        if let Some(chunk) = self.loaded_chunks.get_chunk_by_position(pos_to_oct_pos(pos)){
            chunk.tell(pos_to_local_pos(pos), msg)
        }
    }

    fn load(&mut self, pos: &[Pos]) {
        todo!()
    }

    fn iter_chunks<F>(&self, f: F)
    where F: Fn(&Self::Chunk) {
        todo!()
    }

    fn read_phase<'v, V>(&self, visitors: &'v [V])
    where V: 'v + Send + Sync + world_protocol::VisitorRead<Vox,Self> {
        visitors
            .par_iter()
            .for_each(|visitor|{
                (0..self.loaded_chunks.get_num_chunks())
                    .into_par_iter()
                    .map(|index|self.loaded_chunks.get_chunk(index))
                    .filter(|chunk|{
                        chunk.chunk.visitor_matches(visitor.predicate())
                    })
                    .for_each(|chunk|{
                        chunk.chunk.read_phase(visitor)
                    })
            });
    }

    fn respond_phase<'v, V>(&mut self, visitors: &'v [V])
    where V: 'v + Send + Sync + world_protocol::VisitorRespond<Vox, Self> {
        self.loaded_chunks.iter_chunks()
            .map(|(_, chunk)|chunk)
            .for_each(|chunk|{
                visitors.par_iter()
                .filter(|visitor|chunk.chunk.visitor_matches(visitor.predicate()))
                .for_each(|visitor|{
                    chunk.
                })
            })
            
    }

    fn apply_phase<'v, V>(&mut self, visitors: &'v [V])
    where V: 'v + Send + Sync + world_protocol::VisitorApply<Vox, Self> {
        todo!()
    }

}

impl<Vox: core_obj::Voxel + Send + Sync> Default for Map<Vox>{
    fn default() -> Self {
        Self { 
            loaded_chunks: OctTree::<Chunk<Vox>, OctVec<u32>>::new(),
            all_chunks:  OctTree::<(), OctVec<u32>>::new(),
        }
    }
}

//DEPENDENCY INVERSION
pub trait ChunkTrait<Vox: core_obj::Voxel + Send + Sync>{
    fn get_type(&self, pos: LocalPos) -> Option<Vox::Type>;

    fn tell(&self, pos: LocalPos, msg: VoxelMsg<Vox>);

    fn visitor_matches<'a>(&self, predicated: VisitingPredicate<'a, Vox>) -> bool;

    fn read_phase<V>(&self, visitor: &V) 
    where V: VisitorRead<Vox, Map<Vox>>;

    fn respond_phase<V>(&mut self, visitor: &V) 
    where V: VisitorRespond<Vox,  Map<Vox>>;

    fn apply_phase<V>(&mut self, visitor: &V) 
    where V: VisitorApply<Vox,  Map<Vox>>;
}