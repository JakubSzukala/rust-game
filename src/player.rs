use bevy::{
    prelude::*,
};
use std::collections::HashMap;
use std::time::Duration;

// Components constituting to a player, change it into bundle?

// BEGIN Player components
#[derive(Component)]
pub struct Player;  // empty struct is just a marker for easy extraction

#[derive(Component)]
pub struct Combo {
    pub valid_combos: HashMap<String, String>, // Mapping combo -> attack trait
    pub combo_input_timer: Timer, // Counter for clearing the combo_sequence
    pub combo_sequence: String // Accumulation of user input in time < timeout
}
impl Combo {
    // TODO: Remove hard coded combos sequences
    pub fn new(timeout: u64) -> Combo {
        Combo {
            valid_combos: HashMap::from([
                (String::from("jjjj"), String::from("FireBall")),
                (String::from("kkkk"), String::from("ZaklecieKorwinaMikke"))
            ]),
            combo_input_timer: Timer::new(Duration::from_secs(timeout), false),
            combo_sequence: String::from("")
        }
    }
}

#[derive(Component)]
pub struct Health(pub f32);
    
#[derive(Component)]
pub struct MovementSpeed(pub f32);
// END Player components

/// Create a Player entity and set up it's components
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_entity_id = commands.spawn().id();
    
    commands.entity(player_entity_id)
        .insert(Player)
        .insert(Health(100.0))
        .insert(MovementSpeed(1.0))
        .insert(Combo::new(1))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/player.glb#Scene0"));
        });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 10.7, -10.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    });


    info!("Created a player.");
}


