use bevy::prelude::*;

#[path = "../creature.rs"]
mod creature;

pub mod worm;
pub mod rock;

use creature::Health;

use self::worm::worm_attack;
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
