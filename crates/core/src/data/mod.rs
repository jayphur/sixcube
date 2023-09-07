

pub mod map;
pub mod pos;
mod seed;
pub use seed::*;
mod name;
pub use name::TypeId;

pub const CHUNK_SIZE: usize = 16;