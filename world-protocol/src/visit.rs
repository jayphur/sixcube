use core_obj::{Pos, Type, Voxel, AttrType};
use prelude::*;

use crate::message::VoxelMsg;

// Using visitor pattern.

// MSG PHASE:
//     Sending bs via mpsc to other (ie what to request, etc). Immutable access to all.

// Join all threads, (ensure all msg are sent.)

// RESPOND PHASE:
//     Each voxel will check its queue and do stuff. Mutable access to all of the voxel's queues.
//     The stuff is like responding to "GetAttr"s, etc.
//     Each voxel will use up the remaining stuff in queues and stuff in responses.
//     Mutable access to each.

// no generic Visitor<T> because i don't want to deal with sub trait bs.

///AKA the updater.

pub trait VoxelVisitor<Vox: Voxel, Map: crate::Map<Vox>> {
    fn predicate(&self) ->         &VisitingPredicate<Vox::AttrType>;
    fn predicate_for_mut(&self) -> &VisitingPredicate<Vox::AttrType>;
    fn visit(&self, voxel: VoxelVisit<'_, Vox,Map>);
    fn visit_mut(&self, voxel: VoxelVisitMut<'_, Vox>);
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct VisitingPredicate<A: AttrType> {
    pub with_attributes: Vec<A>,
}

#[derive(Debug)]
pub struct VoxelVisit<'a, Vox: Voxel, Map: crate::Map<Vox>>{
    pub position: Pos,
    pub voxel: &'a Vox,
    pub messages: &'a [VoxelMsg<Vox>],
    pub map: &'a Map,
}

#[derive(Debug)]
pub struct VoxelVisitMut<'a, Vox: Voxel>{
    pub position: Pos,
    pub voxel: &'a mut Vox,
    pub messages: &'a [VoxelMsg<Vox>],
}