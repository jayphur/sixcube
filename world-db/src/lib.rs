use std::marker::PhantomData;

use core_obj::{TypeId, Data, Pos};
use db_protocol::visit::{Message, VoxelVisitor};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use spatialtree::{OctTree, OctVec, Tree};
use chunk::Chunk;

mod chunk;

// Tree tree tree tree tree!!!!!

//Smallest chunk 32^3
//nah maybe 8^3

#[derive(Debug)]
pub struct Map<T, D, M> 
where T: TypeId, D: Data, M: Message 
{
    __marker: PhantomData<(T,D,M)>,
    tree: OctTree<Chunk<T,D,M>, OctVec<u32>>,
    to_update: Vec<usize>
}
impl<T, D, M> db_protocol::Map<T,D,M> for Map<T,D,M>
where T: TypeId, D: Data, M: Message 
{
    fn get_type(&self, pos: Pos) -> Option<T> {
        todo!()
    }

    fn tell(&self, pos: Pos, msg: M) {
        todo!()
    }

    fn visit_each<'v, V>(&self, visitors: &'v [V])
    where V: 'v + Send + Sync + db_protocol::visit::VoxelVisitor<T,D,M, Self> {
        self.to_update.par_iter().map(|index|{
            self.tree.get_chunk(*index)
        }).map(|chunk_container|{
            let pos = chunk_container.position().pos;
            (
                Pos::new(u32_to_i32(pos[0]), u32_to_i32(pos[1]), u32_to_i32(pos[2])),
                &chunk_container.chunk
            )
        }).for_each(|(pos, chunk)|{
            visitors.par_iter().filter(|visitor|{
                if visitor.predicate().with_attributes.len() == 0{
                    true
                } else {
                    visitor.predicate().with_attributes.iter().any(|a|chunk.contains_attr(a))
                }
            }).for_each(|visitor|{
                chunk.get_visited(pos, visitor)
            });
        })
    }

    fn visit_each_mut<'v, V>(&mut self, visitors: &'v [V])
    where V: 'v + Send + Sync + db_protocol::visit::VoxelVisitor<T,D,M, Self> {
        todo!()
    }
}

impl<T, D, M> Default for Map<T,D,M>
where T: TypeId, D: Data, M: Message 
{
    fn default() -> Self {
        Self { 
            __marker: Default::default(), 
            to_update: Default::default(),
            tree: OctTree::<Chunk<T,D,M>, OctVec<u32>>::new(), 
        }
    }
}


const U32_MIDPOINT: u32 = 32768;

pub fn i32_to_u32(val: i32) -> u32{
    if val.is_negative(){
        val.abs() as u32 
    } else {
        val as u32 + U32_MIDPOINT
    }
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

pub trait ChunkTrait<T,D,M>
where T: TypeId, D: Data, M: Message 
{
    fn contains_attr(&self, attr_id: &T::AttrId) -> bool;    
    fn get_visited<V>(&self, pos: Pos, visitor: &V)
    where V: Send + Sync + db_protocol::visit::VoxelVisitor<T,D,M,Map<T,D,M>> ;
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