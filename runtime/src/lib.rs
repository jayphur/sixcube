use data::Data;
use id::TypeId;
use message::Message;
use world_db::Map;

mod data;
mod id;
mod message;

type Dim<'i> = core_obj::Dim<TypeId<'i>, Data, Map<TypeId<'i>, Data, Message>>;
type VisitorPtr<'i> = &'i dyn db_protocol::VoxelIter<'i, TypeId<'i>, Data, Message>;

pub struct ScRuntime<'i> {
    dims: Vec<Dim<'i>>,
}
impl ScRuntime<'_> {}
