// # Plugin library
// The main library file where you define modules within your plugin, as well as where the plugin is initialized at runtime
use manufacture_core::prelude::*;
use manufacture_engine::prelude::*;
use manufacture_engine::ECS::dispatcher::DispatcherBuilder;

pub mod commands;
pub mod comp;
pub mod events;
pub mod resources;
pub mod storage;
pub mod prelude;
pub mod systems;

/// Initialize the plugin
/// 
/// ## DO NOT MODIFY THIS FUNCTION'S SIGNATURE
/// The Plugin Loader will not recognize your plugin if this function is changed in any way
/// 
/// You can add and remove things to initialize, but **do not** change the function's signature
pub fn init_plugin(world: &mut World, disp_build: &mut DispatcherBuilder){
    // -- Components --
    world.register_comp::<comp::SampleComponent>();

    // -- Resources --
    world.register_res::<resources::SampleResource>();

    // -- Events --
    world.register_event::<events::SampleEvent>();

    // -- Systems --
    disp_build.add::<systems::SampleSystem>();
}