use data::Data;
use id::TypeId;
use message::Message;

mod data;
mod id;
mod message;

type Dim<'i> = core_obj::Dim<TypeId<'i>, Data, Map<'i>>;
type Map<'i> = world_db::Map<TypeId<'i>, Data, Message>;

pub struct ScRuntime<'i> {
    dims: Vec<Dim<'i>>,
}
impl ScRuntime<'_> {
    
}
