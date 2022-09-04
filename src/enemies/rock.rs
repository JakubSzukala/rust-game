use bevy::prelude::*;

#[path = "../creature.rs"]
mod creature;
use creature::{Health, BasicAD, MovementSpeed};

use crate::player::Player;

use super::EnemyTraits;

#[derive(Component)]
pub struct RockEnemy;


impl EnemyTraits for RockEnemy {
    fn spawn(
        commands: &mut Commands,
        transform: Transform,
        asset_server: Res<AssetServer>
        ) -> Entity {
        let rock_enemy_entity_id = commands.spawn().id();
        commands.entity(rock_enemy_entity_id)
            .insert(Health(100.0))
            .insert(BasicAD(1.0))
            .insert(MovementSpeed(1.0))
            .insert_bundle(TransformBundle::from(transform))
            .with_children(|parent| {
                parent.spawn_scene(asset_server.load("models/player.glb#Scene0"));
            })
            .id()
    }

    fn follow_player(
        mut rock_q: Query<(&mut Transform, &MovementSpeed), With<RockEnemy>>,
        player_q: Query<&Transform, With<Player>>,
        time: Res<Time>
        ) {
        let player_pos = player_q.single(); 
        for (mut rock_transform, ms) in rock_q.iter_mut() {
            // TODO: Use some A* or smth
            let move_transform = rock_transform.looking_at(
                player_pos.translation,
                Vec3::Y);
            rock_transform.translation += 
                move_transform.translation // direction 
                * ms.0                     // movement spped
                * time.delta_seconds();    // delta
            println!("Rock at {:?}", rock_transform.translation);
        }
    }
}










