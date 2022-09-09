use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct BasicAD(pub f32);
