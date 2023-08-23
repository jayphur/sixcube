use dyn_clone::DynClone;
use std::fmt::Debug;

use crate::data::Name;

/// Type of something. a block / item / etc. Can be applied to "obj"s.
/// A SINGLE static "master" variant of this trait (should) exist for all dyn Type<Obj>s of a specific variant.
/// It can be accessed exclusively behind a 'static reference, or not...
pub trait Type<Obj>: Debug + DynClone {
    fn name(&self) -> &Name;
    fn is_master(&self) -> bool;
    fn new_obj(&self) -> Obj;
}
