use std::marker::PhantomData;

use sc_core::obj::{dim::{self, MapGen}, pos::GlobalPos};
use sc_prelude::*;
mod map;
mod gen;
mod traits;

pub use traits::*;