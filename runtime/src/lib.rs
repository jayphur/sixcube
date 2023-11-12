use core_obj::World;
use dim::Dim;
use voxel::Voxel;

mod data;
mod voxel;
mod dim;
mod init;

pub struct ScRuntime<'i> {
    pub(crate) world: World<Voxel<'i>, Dim<'i>>,
}
impl<'i> ScRuntime<'i> {
    pub fn update(){

    }
}

pub struct ScRuntimeConfig{
    
}