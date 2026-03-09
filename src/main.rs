use manufacture_engine::prelude::*;
use manufacture_sample_plugin::init_plugin;

fn main(){
    let mut world = World::new();
    let mut disp_build = Dispatcher::new();

    manufacture_core::init(&mut world, &mut disp_build);
    init_plugin(&mut world, &mut disp_build);

    let mut dispatcher = disp_build.build();

    dispatcher.dispatch(&mut world);
}