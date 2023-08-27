use std::sync::Arc;
#[derive(Debug, Clone, Default)]
pub struct Name(Arc<String>);

pub mod map;
pub mod pos;
mod seed;
pub use seed::*;

pub const CHUNK_SIZE: usize = 16;