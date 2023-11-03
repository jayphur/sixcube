use core_obj::World;
use dim::Dim;
use voxel::Voxel;
mod data;
mod message;
mod voxel;
mod dim;

pub struct ScRuntime<'i> {
    world: World<Voxel<'i>, Dim<'i>>,
}
impl ScRuntime<'_> {
    pub fn init(){

    }
}

