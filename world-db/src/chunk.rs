use std::marker::PhantomData;

use core_obj::{TypeId, Data, Pos};
use db_protocol::visit::Message;

use crate::ChunkTrait;

#[derive(Debug)]
pub struct Chunk<T,D,M>
where T: TypeId, D: Data, M: Message 
{
    __marker: PhantomData<(T,D,M)>    
}

impl<T,D,M> ChunkTrait<T,D,M> for Chunk<T,D,M>
where T: TypeId, D: Data, M: Message 
{
    fn contains_attr(&self, attr_id: &T::AttrId) -> bool {
        todo!()
    }

    fn get_visited<V>(&self, pos: Pos, visitor: &V)
    where V: Send + Sync + db_protocol::visit::VoxelVisitor<T,D,M,crate::Map<T,D,M>>  {
        todo!()
    }
}