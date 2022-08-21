use bevy::prelude::*;

#[path = "../creature.rs"]
mod creature;
use creature::{Health, BasicAD, MovementSpeed};

use super::EnemyTraits;

pub struct RockEnemy;


pub fn rock_attack() {
    info!("Rock attak! Uga buga!");     
}


pub fn rock_move() {
    info!("Rock tumble!");
}


impl EnemyTraits for RockEnemy {
    fn spawn(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) -> Entity {
        let rock_enemy_entity_id = commands.spawn().id();
        commands.entity(rock_enemy_entity_id)
            .insert(Health(100.0))
            .insert(BasicAD(1.0))
            .insert(MovementSpeed(1.0))
            .insert_bundle(TransformBundle::from(transform))
            .with_children(|parent| {
                parent.spawn_scene(asset_server.load("models/player.glb#Scene0"));
            }).id()
    }
}
