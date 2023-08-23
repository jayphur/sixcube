use sc_prelude::*;
use std::sync::atomic::AtomicU16;

static NEXT_SYSTEM_ID: AtomicU16 = AtomicU16::new(1);

pub struct SystemId(u16);

pub trait System {
    type Component: Component;
    fn id(&self) -> &SystemId;
    fn need_mutable(&self) -> bool; // dictates if i need read_component or use_component
    fn read_component<'a>(&mut self, component: &Self::Component) -> Result<()>;
    fn use_component<'a>(&mut self, component: &mut Self::Component) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct ComponentList();

pub trait Component: Debug + Default {}
