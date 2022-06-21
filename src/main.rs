use bevy::{
    prelude::*,
};


mod input;
use input::InputHandlerPlugin;

mod player;
use player::spawn_player;
use player::Health;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(InputHandlerPlugin)
    .add_startup_system(spawn_player)
    //.add_system(display_players_hp)
    .run();
}

/// Temp function to test if querying works, TODO: remove later
fn display_players_hp(query: Query<(Entity, &Health)>) {
    info!("hi from query"); 
    for (id, health) in query.iter() {
        info!("Entity with id: {} has {} HP.", id.id(), health.0) 
    }
}

