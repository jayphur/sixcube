use sc_core::obj::{dim::{self, MapTrait}, pos::GlobalPos};
use sc_prelude::*;

use crate::map::Map;


#[derive(Debug, Default, Clone)]
pub struct Gen<V, E> where
V: crate::Voxel,
E: crate::Entity, 
{
    _v: PhantomData<V>,
    _e: PhantomData<E>,
}

impl<V, E> dim::MapGen<V,E, Map<V,E>> for Gen<V,E> where
V: crate::Voxel,
E: crate::Entity, 
{
    type Seed = u128;

    fn set_seed(&mut self, seed: &Self::Seed) -> Result<()> {
        todo!()
    }

    fn generate_at(&self, map: &mut Map<V,E>, pos: GlobalPos) -> Result<()> {
        todo!()
    }

    fn generate_at_rad(&self, map: &mut Map<V,E>, center: GlobalPos, radius: u16)  -> Result<()> {
        todo!()
    }

}