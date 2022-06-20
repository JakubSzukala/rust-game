use bevy::{
    prelude::*,
};

// Components constituting to a player, change it into bundle?

// BEGIN Player components
#[derive(Component)]
pub struct Health(pub f32);
    
#[derive(Component)]
pub struct MovementSpeed(f32);
// END Player components

/// Create a Player entity and set up it's components
pub fn spawn_player(mut commands: Commands) {
    let player_entity_id = commands.spawn().id();
    
    commands.entity(player_entity_id)
        .insert(Health(100.0))
        .insert(MovementSpeed(1.0));

    info!("Created a player.");
}


