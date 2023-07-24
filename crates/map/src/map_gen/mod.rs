use sc_core::obj::{dim, pos::GlobalPos};
use sc_prelude::*;

use crate::map::Map;


#[derive(Debug, Default, Clone)]
pub struct MapGen<V, E> where
V: Debug + Default + Clone,
E: Debug + Default + Clone, 
{
    _v: PhantomData<V>,
    _e: PhantomData<E>,
}
impl<V, E> dim::MapGen<V, E, Map<V, E>> for MapGen<V, E>  where
V: Debug + Default + Clone,
E: Debug + Default + Clone, 
{
    type Seed = u16;

    fn set_seed(&mut self, seed: &Self::Seed) -> Result<()> {
        todo!()
    }

    fn generate_at(&self, pos: GlobalPos) -> <Map<V, E> as dim::MapTrait<V,E>>::GenResult {
        todo!()
    }
}