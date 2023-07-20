use std::fmt::Debug;

use super::System;

pub trait Component<S: System>: Debug{
    fn interface_system<'a>(&'a self) -> S::ComponentProxy<'a>;
    fn interface_system_mut<'a>(&'a mut self) -> S::ComponentProxyMut<'a>;
}