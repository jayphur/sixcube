use prelude::*;

use super::{pos::GlobalPos, dim::MapTrait};

pub trait MapGen<V, E, M>: Debug + Default + Clone where
M: MapTrait<V, E>,
{
    type Seed: Clone;

    fn set_seed(&mut self, seed: &Self::Seed) -> Result<()>;
    //TODO: something to do with get(GlobalPos) -> ...
    fn generate_at(&self, pos: GlobalPos) -> M::GenResult;
}