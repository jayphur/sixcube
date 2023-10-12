use super::Map;
use core_obj::{AttrId, Data, Pos, TypeId, Voxel};
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

pub trait VoxelVisitor<T: TypeId, A: AttrId, D: Data, Msg: Message> {
    fn predicate(&self) -> &VisitingPredicate<T, A>;
    fn run_msg<'a, M: Map<'a, T, D, Msg>>(&self, map: &M, voxel: &Voxel<T, D>, pos: Pos);
    fn run_respond(&self, voxel: &mut Voxel<T, D>, pos: Pos);
    fn run_apply(&self, voxel: &mut Voxel<T, D>, pos: Pos);
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct VisitingPredicate<T: TypeId, A: AttrId> {
    pub loaded: bool,
    pub with_attributes: Vec<A>,
    pub of_type: Vec<T>,
}

pub trait Message: Send + Debug + Sync {
    type ResponseRx<T>: ResponseRx<T>;
    type ResponseTx<T>: ResponseTx<T>;
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
