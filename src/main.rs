use bevy::prelude::*;

mod input;
use input::InputHandlerPlugin;

mod player;
use player::spawn_player;

mod user_interface;
use user_interface::UIPlugin;

mod level_builder;
use level_builder::spawn_level;

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
    .add_startup_system(spawn_level)
    .add_startup_system(spawn_player)
    .run();
}



