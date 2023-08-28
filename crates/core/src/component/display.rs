use sc_prelude::*;

use crate::pos::{Pos, GlobalAbsPos};

pub trait Displayable {
    fn get_model(&self) -> Vec<Shape>;
}

pub enum Shape {
    Cube{
        position: GlobalAbsPos,
        color: (u8,u8,u8) // temporary
    }
}
