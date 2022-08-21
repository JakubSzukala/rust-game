use bevy::prelude::*;

#[path = "../creature.rs"]
mod creature;

pub mod worm;
pub mod rock;

//use creature::{Health, BasicAD, MovementSpeed};

//use self::worm::{worm_attack, worm_move};
use self::rock::{rock_attack, rock_move};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_system(worm_attack)
            //.add_system(worm_move)
            .add_system(rock_attack)
            .add_system(rock_move);
    }
}

trait EnemyTraits { // TODO: add a model handle or something
    fn spawn(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) -> Entity;
}



