use core_obj::{TypeId, Data, Pos, Voxel};
use super::Map;
use prelude::*;



// Using visitor pattern.

// MSG PHASE: 
//     Sending bs via mpsc to other (ie what to request, etc). Immutable access to all.

// Join all threads, (ensure all msg are sent.)

// RESPOND PHASE: 
//     Each voxel will check its queue and do stuff. Mutable access to all of the voxel's queues.
//     The stuff is like responding to "GetAttr"s, etc.

// Respond and Apply can happen at the same time... (with blocking for responses and work stealing...) 

// APPLY PHASE: 
//     Each voxel will use up the remaining stuff in queues and stuff in responses.
//     Mutable access to each.

// no generic Visitor<T> because i don't want to deal with sub trait bs.

///AKA the updater. 

pub trait VoxelVisitor<T: TypeId, D: Data>{
    fn predicate(&self) -> &VisitingPredicate<T>; 
    fn run_msg<'a, M: Map<'a, T,D>>(&self, map: &M, voxel: &Voxel<T,D>, pos: Pos);
    fn run_respond(&self, voxel: &mut Voxel<T,D>, pos: Pos);
    fn run_apply(&self, voxel: &mut Voxel<T,D>, pos: Pos);   
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct VisitingPredicate<T: TypeId>{
    pub loaded: bool,
    pub with_attributes: Vec<T::AttrId>, 
    pub of_type: Vec<T>,
}