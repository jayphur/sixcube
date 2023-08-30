#[macro_use]
mod trait_ptr;

mod data;
pub mod obj;
pub use data::*;
pub mod types;
pub mod component {
    pub mod display;
}
