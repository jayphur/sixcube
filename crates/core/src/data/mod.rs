use std::sync::Arc;
#[derive(Debug, Clone, Default)]
pub struct Name(Arc<String>);

pub mod pos;
pub mod map;
mod seed;
pub use seed::*;