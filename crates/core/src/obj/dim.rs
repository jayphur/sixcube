use std::{fmt::Debug, marker::PhantomData};
use prelude::*;
use super::pos::GlobalPos;

/// Dimension
pub struct Dim<MapVox, MapElem, V> where
MapElem: ElemMapTrait<V>,
MapVox: VoxMapTrait<V>,
{
    map_vox: MapVox,
    map_elem: MapElem,
    voxel: PhantomData<V>,
}

pub trait VoxMapTrait<Vox>: Default + Debug + Clone{
    fn generate_nearby(center: GlobalPos, radius: u16) -> Result<()>;
}

pub trait ElemMapTrait<Elem>: Default + Debug + Clone{
    
}
