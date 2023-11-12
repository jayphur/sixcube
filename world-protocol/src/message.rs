use core_obj::{AttrType, Voxel, Dim, ActionType, Action, ActionResult};

#[derive(Debug)]
pub enum VoxelMsg<V: Voxel>{
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
}