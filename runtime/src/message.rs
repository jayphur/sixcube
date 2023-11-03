use world_protocol::visit::{ResponseRx, ResponseTx};

#[derive(Debug)]
pub enum Message {}
impl world_protocol::visit::Message for Message {
    type ResponseRx<T> = MsgResponse<T>;
    type ResponseTx<T> = MsgResponder<T>;
}

pub struct MsgResponse<T>(oneshot::Receiver<T>);
pub struct MsgResponder<T>(oneshot::Sender<T>);

impl<T> ResponseTx<T> for MsgResponder<T> {}
impl<T> ResponseRx<T> for MsgResponse<T> {}
