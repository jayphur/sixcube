use std::mem::take;

use async_trait::async_trait;
use sc_prelude::*;

use crate::obj::dim::MapError;

#[derive(Debug)]
pub enum LoadOpt<T,> 
where
T: Debug + Default + Send, 
{
    Loaded(T),
    Unloaded(FsPointer<T>),
}
impl<T> LoadOpt<T>
where
T: Debug + Default + Send, 
{
    pub fn new(t: T) -> Self{
        Self::Loaded(t)
    }
    pub async fn try_load(&mut self) -> Result<()>{
        match take(self){
            LoadOpt::Loaded(_) => {
                Ok(())
            },
            LoadOpt::Unloaded(loader) => {
                *self = Self::Loaded(loader.load().await?);
                Ok(())
            },
        }
    }
    pub fn unwrap_loaded(&self) -> Result<&T>{
        match self {
            LoadOpt::Loaded(t) => Ok(t),
            LoadOpt::Unloaded(_) => Err(MapError::Unloaded.into()),
        }
    }

    pub fn unwrap_loaded_mut(&mut self) -> Result<&mut T>{
        match self {
            LoadOpt::Loaded(t) => Ok(t),
            LoadOpt::Unloaded(_) => Err(MapError::Unloaded.into()),
        }
    }
}
impl<T> Default for LoadOpt<T> 
where
T: Debug + Default + Send, 
{
    fn default() -> Self {
        Self::Loaded(Default::default())
    }
}
#[async_trait]
pub trait Loads<T>{
    async fn load(self) -> Result<T>;
}

#[derive(Debug, Default)]
pub struct FsPointer<T: Send>(T); //TODO: this is a temporary "loading". make it actually load T and not just store T
#[async_trait]
impl<T: Send> Loads<T> for FsPointer<T>{
    async fn load(self) -> Result<T> {
        Ok(self.0)
    }
}