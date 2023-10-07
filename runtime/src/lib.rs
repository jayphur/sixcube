use data::Data;
use id::TypeId;
use world_db::Map;

mod data;
mod id;

type Dim<'i> = core_obj::Dim<TypeId<'i>, Data, Map<TypeId<'i>, Data>>;
type VisitorPtr<'i> = &'i dyn db_protocol::VoxelIter<'i, TypeId<'i>, Data>;

pub struct ScRuntime<'i>{
    dims: Vec<Dim<'i>>,

}
impl ScRuntime<'_>{

}