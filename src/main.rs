use bevy::{
    prelude::*,
    window::ReceivedCharacter
};


mod input;
use input::InputHandler;
use input::AgentStub;

fn main() {
    App::new()
    .add_system(hello_world)
    .add_plugins(DefaultPlugins)
    .add_system(input_handler_wrapper)
    .run();
}

// This is crap, it's only for testing TODO read more on bevy app struct
fn input_handler_wrapper(keyboard_input: Res<Input<KeyCode>>) {
    let agent = AgentStub {
        x: 1,
        y: 2,
        lvl: 69,
        speed: 96
    };
    let input_handler = InputHandler {agent: agent};

    let command = input_handler.handle_inputs(keyboard_input);
    command.execute();
}

fn hello_world() {
    println!("Hi");
}


