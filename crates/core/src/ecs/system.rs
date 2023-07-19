use std::sync::atomic::AtomicU16;

static NEXT_SYSTEM_ID: AtomicU16 = AtomicU16::new(1);
pub struct SystemId(u16);

pub trait System{
    type CompRef<'a>;      // Define what a user of the system must do.
    type CompRefMut<'a>;

    fn id(&self) -> &SystemId;
    fn read_component<'a>(&self, component: Self::CompRef<'a>); //TODO: return Result<()> ???
    fn use_component<'a>(&mut self, component: Self::CompRefMut<'a>); //TODO: return Result<()> ???
}
