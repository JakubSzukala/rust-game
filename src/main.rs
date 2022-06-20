use bevy::{
    prelude::*,
    window::ReceivedCharacter
};


mod input;

fn main() {
    App::new()
    .add_system(hello_world)
    .add_plugins(DefaultPlugins)
    .add_system(input_handler_wrapper)
    .run();
}


fn hello_world() {
    println!("Hi");
}


