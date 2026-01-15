// # Plugin library
// The main library file where you define modules within your plugin, as well as where the plugin is initialized at runtime
use manufacture_core::prelude::*;

pub mod commands;
pub mod comp;
pub mod events;
pub mod resources;
pub mod storage;
pub mod prelude;
pub mod systems;