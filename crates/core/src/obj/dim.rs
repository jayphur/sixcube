use std::{fmt::Debug, marker::PhantomData};

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

}

pub trait ElemMapTrait<Elem>: Default + Debug + Clone{

}
