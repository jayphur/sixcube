use crate::pos::GlobalAbsPos;

pub trait Displayable {
    fn get_model(&self) -> Model;
    fn read(&mut self, /* read stuff? */); // idea: only available with mutable access to this guy
}


pub enum Model {
    Cube{
        position: GlobalAbsPos,
        color: (u8,u8,u8) // temporary
    }
}

