use bevy::{
    prelude::*,
};


mod input;
use input::InputHandlerPlugin;

fn main() {
    App::new()
    //.add_system(hello_world)
    .add_plugins(DefaultPlugins)
    .add_plugin(InputHandlerPlugin)
    .run();
}


fn hello_world() {
    println!("Hi");
}


