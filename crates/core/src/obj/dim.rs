use std::{fmt::Debug, marker::PhantomData};

/// Dimension
pub struct Dim<Map, MapGen, Vox> where
Map: ElemMapTrait<Vox, MapGen> + VoxMapTrait<Vox, MapGen>,
{
    map: Map,
    //markers
    voxel: PhantomData<Vox>,
    map_gen: MapGen,
}

pub trait VoxMapTrait<Vox, Gen>: Default + Debug{
    type Proxy;

}

pub trait ElemMapTrait<Elem, Gen>: Default + Debug{
    type Proxy;
}
