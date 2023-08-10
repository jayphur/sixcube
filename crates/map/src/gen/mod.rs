use sc_core::{obj::dim::{self}, pos::GlobalPos};
use sc_prelude::*;

use crate::map::Map;


#[derive(Debug, Default, Clone)]
pub struct Gen<V, E>{
    _v: PhantomData<V>,
    _e: PhantomData<E>,
}

impl<V: Debug + Default, E> Gen<V,E>{
    fn set_seed(&mut self, seed: &u32) -> Result<()> {
        todo!()
    }

    fn generate_at(&self, map: &mut Map<V,E>, pos: GlobalPos) -> Result<()> {
        todo!()
    }

    fn generate_at_rad(&self, map: &mut Map<V,E>, center: GlobalPos, radius: u16)  -> Result<()> {
        todo!()
    }

}