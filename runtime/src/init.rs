use std::marker::PhantomData;

use crate::{ScRuntime, ScRuntimeConfig, voxel::Voxel, dim::Dim};
use core_obj::World;
use prelude::*;

pub fn start_runtime<F>(config: ScRuntimeConfig, body: F) -> Result<()> 
    where F: for<'i> FnOnce(ScRuntime<'i>) -> Result<()>
{

    let world = setup_world();
    //It's all set up!
    body(ScRuntime{
        world,
    })
}

fn setup_types<'i>(/* stuff needed */){ //TODO: wtf does this return
    
}

fn setup_world<'i>(/* stuff needed */) -> World<Voxel<'i>, Dim<'i>>{
    World { 
        dims: Vec::new(), 
        _m: PhantomData 
    }
}
