use std::fmt::Debug;
use crate::{data::Name, obj::{voxel::Voxel, element::Element, dim::{Dim, MapTrait}}};

/// Type of something. a block / item / etc. Can be applied to "obj"s.
pub trait Type: Debug{
    type Obj: Debug;
    
    fn name(&'static self) -> &Name;
    fn new_obj(&'static self) -> Self::Obj;
}

// the type trait ready for specific objects. They should be able to return some info with using a `&'static self`

pub trait VoxelType: Type<Obj=Voxel>{

}

pub trait ElementType: Type<Obj=Element>{
    
}

pub trait DimType<Map>: Type<Obj = Dim<Map>> where
Map: MapTrait<Voxel, Element> + 'static
{

}