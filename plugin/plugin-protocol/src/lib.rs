use std::marker::PhantomData;

use core_obj::{Runtime, Value};

pub struct Plugin<>{
    pub pretty_name: String,
    pub name: String,
    pub voxels: Vec<VoxelDef>,
}

pub struct VoxelDef<>{
    pub name: String,
    pub static_attr: Vec<(String, Value)>,
    pub react_fn: Option<()>, //TODO
}

pub struct Host<R: Runtime>{
    temp: PhantomData<R>
}