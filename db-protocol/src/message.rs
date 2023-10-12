use std::marker::PhantomData;

use core_obj::{TypeId, AttrValue, Data, Action};
use prelude::*;

pub enum Message<T: TypeId, D: Data>{
    Query(T::AttrId, Responder<AttrValue>, PhantomData<D>),
    Action(Action<T>),
    TryAction(Action<T>, Responder<bool>),
}
impl<T: TypeId, D: Data> Message<T,D>{
    pub fn query(attr: T::AttrId) -> (Self, Response<AttrValue>){
        let (tx, rx) = oneshot::channel();
        (
            Message::Query(attr, Responder(tx), PhantomData),
            Response(rx)
        )
    }
    pub fn action(action: Action<T>) -> Self{
        Message::Action(action)
    }
    pub fn try_action(action: Action<T>) -> (Self, Response<bool>){
        let (tx, rx) = oneshot::channel();
        (
            Message::TryAction(action, Responder(tx)),
            Response(rx)
        )
    }
}





#[derive(Debug)]
pub struct Responder<T: Send>(oneshot::Sender<T>);
impl<T: Send> Responder<T>{
    pub fn respond(self, t: T){
        self.0.send(t).unwrap()
    }
}
#[derive(Debug)]
pub struct Response<T: Send>(oneshot::Receiver<T>);
impl<T: Send> Response<T>{
    pub fn try_recv(&self) -> Option<T>{
        match self.0.try_recv(){
            Ok(val) => Some(val),
            Err(oneshot::TryRecvError::Empty) => None,
            Err(oneshot::TryRecvError::Disconnected) => None, //FIXME: handle differently?
        }
    }
    pub fn recv_blocking(self) -> T{
        self.0.recv().unwrap()
    }
}