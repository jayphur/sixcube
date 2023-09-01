use crate::{pos::GlobalPos, obj::{voxel::VoxelTypePtr, dim::{Dim, MapTrait}}};

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
    map_updates: L,
}
impl<L: MapListener> DimDisplay<L>{
    fn big_load_map<M: MapTrait>(&mut self, map: &M){
        todo!()
    }
    fn big_load_dim(&mut self, dim: &Dim){
        self.big_load_map(&dim.map);
        todo!()
    }
}


pub trait ListenableMap{
    type Listener: MapListener;
    fn new_rx(&mut self) -> Self::Listener;
}
pub trait MapListener {
    fn block_rx(&self) -> MapUpdate;
    fn try_rx(&self) -> Option<MapUpdate>;
}
pub enum MapUpdate{
    Removed(GlobalPos),
    Added(GlobalPos, VoxelTypePtr)
}
