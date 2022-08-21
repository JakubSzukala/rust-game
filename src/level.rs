use bevy::{
    prelude::*,
    gltf::Gltf,
    render::mesh::{Indices, VertexAttributeValues},
};

use bevy_rapier3d::{prelude::*, rapier::prelude::ColliderBuilder};


#[derive(Component)]
pub struct Level;  // empty struct is just a marker for easy extraction

// Helper resource for tracking meshes that are used for collisions
struct CollisionMesh{
    handle : Handle<Mesh>
}

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


}

fn spawn_level( mut commands: Commands,
                    asset_server: Res<AssetServer>,
                ) {
    let level_collision_mesh: Handle<Mesh> = asset_server.load("map/test/collision.glb#Mesh0/Primitive0");
    //commands.insert_resource(CollisionMesh{handle : level_collision_mesh});

    let level_entity_id = commands.spawn().id();

    
    commands.entity(level_entity_id)
        .insert(Level)
        .insert(RigidBody::Fixed)
        .insert(level_collision_mesh.clone_weak())
        .insert(Collider::ball(3.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("map/test/model.glb#Scene0"));

        });
    commands.insert_resource(level_collision_mesh);


}

fn update_level_collision(
    mut ev_asset: EventReader<AssetEvent<Mesh>>,
    mut assets: ResMut<Assets<Mesh>>,
    mut p: Query<&mut Collider, With<Level>>,
    r: Query<&Handle<Mesh>, With<Level>>,

) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                    let level_collision_handle = r.single();
                    let mut level_collider = p.single_mut();


                    let mesh = assets.get_mut(handle).unwrap();
                    // ^ unwrap is OK, because we know it is loaded now

                    if !Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).is_none(){

                        if *level_collision_handle == *handle {
                            *level_collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
                        }
                    }

            }
            AssetEvent::Modified { handle : _} => {

            }
            AssetEvent::Removed { handle : _} => {
                // an image was unloaded
            }
        }
    }
}
