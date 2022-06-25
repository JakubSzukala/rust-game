use bevy::prelude::*;

mod input;
use input::InputHandlerPlugin;

mod player;
use player::spawn_player;

mod user_interface;
use user_interface::UIPlugin;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
            title: "Rust Game".to_string(),
            width: 500.,
            height: 300.,
            ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(InputHandlerPlugin)
    .add_plugin(UIPlugin)
    .add_startup_system(spawn_player)
    .run();
}



