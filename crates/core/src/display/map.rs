use crate::{pos::GlobalPos, obj::voxel::VoxelTypePtr};

/// A representation of a portion of a map
pub struct MapRep{
    
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