use bevy::prelude::*;

#[path = "../creature.rs"]
mod creature;

pub mod worm;
pub mod rock;

use self::rock::{RockEnemy, rock_follow_player};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(rock_follow_player)
            .add_startup_system(RockEnemy::<spawn>);
        // TODO !!!!!!!!!!!!!!!!!!!!
        // Use this as example on how to implement EnemiesPlugin to use traits
        // for attack, move etc
        // https://github.com/bevyengine/bevy/blob/main/crates/bevy_render/src/camera/projection.rs
        // https://bevy-cheatbook.github.io/cookbook/custom-projection.html
        // https://bevy-cheatbook.github.io/patterns/generic-systems.html
    }
}

trait EnemyTraits { 
    fn spawn(
        commands: &mut Commands,
        transform: Transform,
        asset_server: Res<AssetServer>) -> Entity;
}


fn spawn_enemies() {
    RockEnemy::spawn();
}



