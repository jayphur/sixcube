use core_obj::Runtime;

/// There is no evaluation of a voxel's type, it is definite; there doesn't need to be a QueryType
#[derive(Debug)]
pub enum VoxelMsg<R: Runtime>{
    QueryAttr{
        attr_type: R::AttrType,
        tx: oneshot::Sender<core_obj::Value>,
    },
}
impl<R: Runtime> VoxelMsg<R>{
    pub fn new_query_attr(attr_type: R::AttrType) -> (Self, oneshot::Receiver<core_obj::Value>){
        let (tx,rx) = oneshot::channel();
        (Self::QueryAttr { attr_type, tx }, rx)
    }
}