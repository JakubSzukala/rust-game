use bevy::prelude::*;

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>
    ) {
    info!("Spawning a scene...");
    let level_scene = asset_server.load(
        "scenes/temp_scene_rust_game.glb#Scene0");

    // Camera TODO: remove it later so camera is attached to player
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(5.0, 2.0, 2.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
    
    /// Lightning
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    
    commands.spawn_scene(level_scene);
}


