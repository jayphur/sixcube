use core_obj::{Pos, Type, Voxel, AttrType};
use prelude::*;

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
    fn visit_mut(&self, voxel: VoxelVisitMut<'_, Vox,Map::Msg>);
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct VisitingPredicate<A: AttrType> {
    pub with_attributes: Vec<A>,
}

#[derive(Debug)]
pub struct VoxelVisit<'a, Vox: Voxel, Map: crate::Map<Vox>>{
    pub position: Pos,
    pub voxel: &'a Vox,
    pub messages: &'a [Map::Msg],
    pub map: &'a Map,
}

#[derive(Debug)]
pub struct VoxelVisitMut<'a, Vox: Voxel, Msg: Message>{
    pub position: Pos,
    pub voxel: &'a mut Vox,
    pub messages: &'a [Msg],
}

pub trait Message: Send + Debug + Sync {
    type ResponseRx<T>: ResponseRx<T>;
    type ResponseTx<T>: ResponseTx<T>;
    /// How this message should be resolved if the recipient does not exist.
    /// 
    /// By default: do nothing.
    fn handle_empty(&self) {}
}
pub trait ResponseRx<T> {}
pub trait ResponseTx<T> {}




pub mod fake_types {
    use super::{Message, ResponseRx, ResponseTx};

    #[derive(Default, Debug, Clone, Copy)]
    pub struct FakeMessage {}
    impl Message for FakeMessage {
        type ResponseRx<T> = FakeResponseRx;
        type ResponseTx<T> = FakeResponseTx;
    }
    pub struct FakeResponseTx();
    impl<T> ResponseTx<T> for FakeResponseTx {}
    pub struct FakeResponseRx();
    impl<T> ResponseRx<T> for FakeResponseRx {}
}
