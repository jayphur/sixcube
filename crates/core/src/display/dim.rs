use crate::obj::dim::Dim;
use super::map::MapListener;

pub trait VoxelDisplayInfo{
    fn translucent(&self) -> bool;
}

pub struct Sides<T>{
    up: T,
    down: T,
    north: T,
    east: T,
    south: T,
    west: T,
}

/// A representation of a dimension.
pub struct DimDisplay<L: MapListener>{
    map_update_rx: L,
}
impl<L: MapListener> DimDisplay<L>{
    fn full_load_dim(&mut self, dim: &Dim){
        todo!()
    }
}