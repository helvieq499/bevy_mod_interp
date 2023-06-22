#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::multiple_crate_versions)]

mod component;
mod plugin;
mod system;

pub mod prelude;
pub use prelude::*;
