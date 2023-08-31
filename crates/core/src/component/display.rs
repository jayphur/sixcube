pub trait Displayable {
    fn get_model(&self) -> ();
    fn read(&mut self, /* read stuff? */); // idea: only available with mutable access to this guy
}
