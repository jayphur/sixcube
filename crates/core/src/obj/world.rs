use crate::display::world::WorldDisplay;

use super::dim::Dim;

#[derive(Debug)]
pub struct World {
    //pub dims: Vec<Dim>,
    pub dim: Dim, //TODO: support for multiple dims later, just one for now.

}