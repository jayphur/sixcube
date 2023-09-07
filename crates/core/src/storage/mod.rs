use serde::Serialize;
use crate::{obj::world::World, types::TypePtrList, TypeId};
use self::dictionary::StringDictionary;
use sc_prelude::*;

mod voxel;
mod error;
mod dictionary;

trait Package<Instance>{
    type Packaged: Serialize;
    fn package(&mut self, this: Instance) -> Self::Packaged;
}
trait Unpackage<Input>{
    type Output;
    fn unpackage<'a>(&'a mut self, i: Input, list: &'a TypePtrList) -> Result<Self::Output>;
}

#[derive(Debug, Default, Serialize)]
pub struct WorldStorage{
    voxel_type_ids: StringDictionary,
    //packaged_world: ...
}
impl WorldStorage{
    pub fn new(world: World) -> Self{
        todo!()
    }
    /// A price-y function to run. Creates all new TypeIds with cloning/allocation.
    pub fn voxel_type_ids(& self) -> Vec<TypeId>{
        self.voxel_type_ids.inner().iter().map(|string| string.clone().into()).collect()
    }
    pub fn unpack<'a>(self, list: &'a TypePtrList) -> World{
        todo!()
    }
}