use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod input;
use input::InputHandlerPlugin;

mod player;
use player::PlayerSetupPlugin;

mod user_interface;
use user_interface::UIPlugin;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
            title: "Rust Game".to_string(),
            width: 800.,
            height: 600.,
            ..default()
    })
    .add_plugins(DefaultPlugins) // Bevy plugin
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default()) // Rapier plugin
    .add_plugin(RapierDebugRenderPlugin::default()) // Rapier plugin
    .add_plugin(InputHandlerPlugin)
    .add_plugin(UIPlugin)
    .add_plugin(PlayerSetupPlugin)
    .add_startup_system(setup_physics)
    .add_system(print_ball_altitude)
    .run();
}



fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
