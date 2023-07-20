use std::{fmt::Debug, marker::PhantomData};
use prelude::*;
use super::pos::GlobalPos;

/// Dimension
pub struct Dim<MapVox, MapElem, MapGen, Vox> where
MapElem: ElemMapTrait<Vox, MapGen>,
MapVox: VoxMapTrait<Vox, MapGen>,
{
    map_vox: MapVox,
    map_elem: MapElem,
    //markers
    voxel: PhantomData<Vox>,
    map_gen: MapGen,
}

pub trait VoxMapTrait<Vox, Gen>: Default + Debug + Clone{
    type MapProxy;
    /// Should generate at least this area.
    fn generate_at(center: GlobalPos, radius: u16, world_gen: &Gen) -> Result<()>;
}

pub trait ElemMapTrait<Elem, Gen>: Default + Debug + Clone{
    
}
