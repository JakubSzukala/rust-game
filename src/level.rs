use bevy::{
    prelude::*,
    gltf::Gltf
};

use bevy_rapier3d::{prelude::*, rapier::prelude::ColliderBuilder};

#[derive(Component)]
pub struct Level;  // empty struct is just a marker for easy extraction
                   //


// Helper resource for tracking meshes that are used for collisions
struct CollisionMesh(Handle<Mesh>);

pub struct LevelLoadPlugin;
impl Plugin for LevelLoadPlugin{
    fn build(&self, app: &mut App) {
        info!("Hi from app, seting up level systems");
        app.add_startup_system(load_level_models);
        app.add_startup_system(spawn_level);
    }
}

fn load_level_models( mut commands: Commands, asset_server: Res<AssetServer>) {
    let level_scene: Handle<Scene> = asset_server.load("map/test/model.glb#Scene0");
    let level_collision_mesh: Handle<Mesh> = asset_server.load("map/test/collision.glb#Mesh0");
    commands.insert_resource(level_scene);
    commands.insert_resource(CollisionMesh(level_collision_mesh));

}

fn spawn_level( mut commands: Commands,
                    asset_server: Res<AssetServer>,
                    collision_mesh_res: Res<CollisionMesh>,
                    meshes_res: Res<Assets<Mesh>>,
                ) {

    if let Some(collision_mesh) = meshes_res.get(&collision_mesh_res.0){
        
        let level_collider = Collider::from_bevy_mesh(collision_mesh, &ComputedColliderShape::TriMesh);
        
        commands.spawn()
            .insert(RigidBody::Fixed)
            .insert(level_collider.unwrap());

    }





    let level_entity_id = commands.spawn().id();

    
    commands.entity(level_entity_id)
        .insert(Level)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("map/test/model.glb#Scene0"));
        });

}
