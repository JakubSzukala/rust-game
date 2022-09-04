use bevy::prelude::*;

#[path = "../creature.rs"]
mod creature;
use creature::MovementSpeed;

pub mod worm;
pub mod rock;

use crate::player::Player;

//use self::rock::{RockEnemy};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyActionsPlugin<T: EnemyActions>;
// Implement Plugin trait on any EnemyActionsPlugin type that implements EnemyActions trait
// Conditional impl
impl<T: EnemyActions> Plugin for EnemyActionsPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .add_system(T::follow_player)
            .add_startup_system(T::spawn);
    }
}

trait EnemyActions {
    fn spawn(
        commands: &mut Commands,
        transform: Transform,
        asset_server: Res<AssetServer>) -> Entity;
    fn follow_player(
        enemy_q: Query<(&Transform, &MovementSpeed), With<Enemy>>,
        player_q: Query<&Transform, With<Player>>);
}

#[derive(Component)]
pub struct SomeEnem;

impl EnemyActions for SomeEnem {
    fn spawn(
        commands: &mut Commands,
        transform: Transform,
        asset_server: Res<AssetServer>) -> Entity {
        println!("hi spawn here");
    }
    fn follow_player(
        enemy_q: Query<(&Transform, &MovementSpeed), With<Enemy>>,
        player_q: Query<&Transform, With<Player>>) {
        println!("Hi following here");
    }

}


