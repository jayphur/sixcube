use std::ops::DerefMut;

use crate::obj::world::World;

use super::dim::DimDisplay;

pub struct WorldDisplay{
    dim: DimDisplay, //TODO: vector instead of just one.
}
impl WorldDisplay{
    pub fn new<W>(mut world: W) -> Self
    where
    W: DerefMut<Target = World>
    {
        Self { 
            dim: DimDisplay::new(&mut (*world).dim)
        }
    }
}