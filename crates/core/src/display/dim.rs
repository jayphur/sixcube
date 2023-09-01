use std::ops::DerefMut;

use crate::{obj::dim::Dim, map::Listener};
use super::map::{MapListener, MapDisplay};

/// A representation of a dimension.
pub struct DimDisplay{
    map: MapDisplay,
}
impl DimDisplay{
    pub fn new<D>(mut dim: D) -> Self 
    where
    D: DerefMut<Target = Dim>
    {
        Self { 
            map: MapDisplay::new(&mut (*dim).map) 
        }
    }
    pub fn full_load_dim(&mut self, dim: &Dim){
        todo!()
    }
}