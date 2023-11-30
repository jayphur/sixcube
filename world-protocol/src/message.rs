use core_obj::{AttrType, Voxel, Dim, ActionType, Action, ActionResult};

/// There is no evaluation of a voxel's type, it is definite; there doesn't need to be a QueryType
#[derive(Debug)]
pub enum VoxelMsg<V: Voxel>{
    QueryAttr{
        attr_type: V::AttrType,
        tx: oneshot::Sender<core_obj::Value>,
    },
    Action{
        action: Action<V::ActionType>,
        result_tx: oneshot::Sender<ActionResult>,
    },
}
impl<V: Voxel> VoxelMsg<V>{
    pub fn new_action(action: Action<V::ActionType>) -> (Self, oneshot::Receiver<ActionResult>){
        let (result_tx,rx) = oneshot::channel();
        (Self::Action { action, result_tx },rx)
    }
    pub fn new_query_attr(attr_type: V::AttrType) -> (Self, oneshot::Receiver<core_obj::Value>){
        let (tx,rx) = oneshot::channel();
        (Self::QueryAttr { attr_type, tx }, rx)
    }
}