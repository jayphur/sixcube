use sc_prelude::*;
use std::sync::mpsc;

pub trait Displayable {
    fn get_model(&self) -> Vec<Shape>;
    fn send_shapes(&self, tx: &mpsc::Sender<Shape>) -> Result<()> {
        for shape in self.get_model() {
            tx.send(shape)?;
        }
        Ok(())
    }
}

pub enum Shape {}
