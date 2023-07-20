use std::sync::atomic::AtomicU16;
use prelude::*;
static NEXT_SYSTEM_ID: AtomicU16 = AtomicU16::new(1);

pub struct SystemId(u16);

pub trait System{
    type ComponentProxy<'a>;      // Define what a user of the system must do.
    type ComponentProxyMut<'a>;

    fn id(&self) -> &SystemId;
    fn need_mutable(&self) -> bool; // dictates if i need read_component or use_component
    fn read_component<'a>(&self, component: Self::ComponentProxy<'a>) -> Result<()>; //TODO: return Result<()> ???
    fn use_component<'a>(&mut self, component: Self::ComponentProxyMut<'a>) -> Result<()>; //TODO: return Result<()> ???
}
