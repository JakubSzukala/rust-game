use bevy::{
    prelude::*,
    gltf::Gltf,
    render::mesh::{Indices, VertexAttributeValues},
};

use bevy_rapier3d::{prelude::*, rapier::prelude::ColliderBuilder};


#[derive(Component)]
pub struct Level;  // empty struct is just a marker for easy extraction

// Helper resource for tracking meshes that are used for collisions
struct CollisionMesh(Handle<Mesh>);

pub struct LevelLoadPlugin;
impl Plugin for LevelLoadPlugin{
    fn build(&self, app: &mut App) {
        info!("Hi from app, seting up level systems");
        app.add_startup_system(load_level_models);
        app.add_startup_system(spawn_level);
        app.add_system(update_level_collision);
    }
}

fn load_level_models( mut commands: Commands, asset_server: Res<AssetServer>) {
    let level_collision_mesh: Handle<Mesh> = asset_server.load("map/test/collision.glb#Mesh0");
    commands.insert_resource(level_collision_mesh);

}

fn spawn_level( mut commands: Commands,
                    asset_server: Res<AssetServer>,
                ) {

    let level_entity_id = commands.spawn().id();

    
    commands.entity(level_entity_id)
        .insert(Level)
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(3.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("map/test/model.glb#Scene0"));
        });

}

fn update_level_collision(
    mut ev_asset: EventReader<AssetEvent<Mesh>>,
    mut assets: ResMut<Assets<Mesh>>,
    mut p: Query<&mut Collider, With<Level>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                // an image was modified
                // a texture was just loaded or changed!

                // WARNING: this mutable access will cause another
                // AssetEvent (Modified) to be emitted!
                let mesh = assets.get_mut(handle).unwrap();
                // ^ unwrap is OK, because we know it is loaded now

                let mut level = p.single_mut();

                //*level = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
                if !Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).is_none(){
                    *level = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
                }

            }
            AssetEvent::Modified { handle } => {

            }
            AssetEvent::Removed { handle } => {
                // an image was unloaded
            }
        }
    }
}
