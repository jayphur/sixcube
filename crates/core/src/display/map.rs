use std::ops::DerefMut;

use ndarray::Array3;
use crate::{pos::GlobalPos, obj::{voxel::{VoxelTypePtr, Voxel}, element::Element}, map::{Map, Listener}};

/// The graphical representation of a portion of a map
pub struct MapDisplay{
    nearby_voxels: Array3<VoxelRep>,
    listener: Listener,
}
impl MapDisplay{
    pub fn new<M>(map: M) -> Self
    where
    M: DerefMut<Target = Map<Voxel, Element>>
    {
        Self { 
            nearby_voxels: todo!(),
            listener: map.new_listener(), 
        }
    }
}
pub trait ListenableMap{
    type Listener: MapListener;
    fn new_listener(&mut self) -> Self::Listener;
}
pub trait MapListener {
    fn block_rx(&self) -> MapUpdate;
    fn try_rx(&self) -> Option<MapUpdate>;
}
pub enum MapUpdate{
    Removed(GlobalPos),
    Added(GlobalPos, VoxelTypePtr)
}

/// The graphical representation of a voxel, not *inherently* indicative of 
/// the actual value stored in the map.
pub enum VoxelRep{
    NotVisible,
    Visible{
        exposed_sides: Sides<bool>,
    }
}

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
