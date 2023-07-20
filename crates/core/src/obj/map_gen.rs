use prelude::*;

use super::pos::GlobalPos;

pub trait MapGenerator<Vox, Elem>: Debug + Default + Clone{
    type Seed: Clone;
    type GenerationResult;

    fn set_seed(&mut self, seed: &Self::Seed) -> Result<()>;
    //TODO: something to do with get(GlobalPos) -> ...
    fn generate_at(&self, pos: GlobalPos) -> Self::GenerationResult;
}