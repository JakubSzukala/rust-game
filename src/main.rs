use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod input;
use input::InputHandlerPlugin;

mod player;
use player::PlayerSetupPlugin;

mod user_interface;
use user_interface::UIPlugin;

mod level;
use level::LevelLoadPlugin;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
            title: "Rust Game".to_string(),
            width: 800.,
            height: 600.,
            ..default()
    })
    .add_plugins(DefaultPlugins) // Bevy plugin
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default()) // Rapier plugin
    .add_plugin(RapierDebugRenderPlugin::default()) // Rapier plugin
    .add_plugin(InputHandlerPlugin)
    .add_plugin(UIPlugin)
    .add_plugin(PlayerSetupPlugin)
    .add_plugin(LevelLoadPlugin)
    .run();
}
