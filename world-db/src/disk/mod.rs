use std::{fs, io::Write, marker::PhantomData, path::Path, sync::Arc};

use serde::Serialize;

use core_obj::Registrar;
use prelude::*;

pub mod rle;
mod region;

pub struct MapFile<R: Registrar>{
    pub(crate) path: Arc<Path>,    
    __marker: PhantomData<R>, //What

}
impl<R: Registrar> MapFile<R>{
    async fn init(path: Arc<Path>, registrar: &R) -> Result<Self>{
        fs::metadata(&path)?;

        Ok(Self { 
            path,
            __marker: PhantomData, 
        })
    }

}


//lookup table maybe...
