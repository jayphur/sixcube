use std::marker::PhantomData;

#[derive(Default, Debug)]
pub struct Msg{

}
impl world_protocol::visit::Message for Msg{
    type ResponseRx<T> = ResponseRx<T>;

    type ResponseTx<T> = ResponseTx<T>;
}

pub struct ResponseRx<T>{
    __phantom: PhantomData<T>,
}
impl<T> world_protocol::visit::ResponseRx<T> for ResponseRx<T>{

}

pub struct ResponseTx<T>{
    __phantom: PhantomData<T>,
}
impl<T> world_protocol::visit::ResponseTx<T> for ResponseTx<T>{

}
