#[macro_use]
mod trait_ptr;

mod data;
pub mod obj;
pub use data::*;
pub mod types;
pub mod display {
    pub mod dim;
    pub mod map;
    pub mod world;
}
pub mod storage;