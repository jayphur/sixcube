use std::sync::Arc;
#[derive(Debug, Clone, Default)]
pub struct Name(Arc<String>);

pub mod map;
pub mod pos;
mod seed;
pub use seed::*;

pub const CHUNK_SIZE: i16 = 16;
pub const CHUNK_SIZE_U: usize = 16;