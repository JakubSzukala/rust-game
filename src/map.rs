use bevy::{
    prelude::*,
};

#[derive(Component)]
pub struct Map;  // empty struct is just a marker for easy extraction

pub struct MapLoadPlugin;
impl Plugin for MapLoadPlugin{
    fn build(&self, app: &mut App) {
        info!("Hi from app, seting up map systems");
        app.add_startup_system(load_map_model);
        app.add_startup_system(load_map_collision);
    }
}

pub fn load_map_model( mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_entity_id = commands.spawn().id();
    
    commands.entity(map_entity_id)
        .insert(Map)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("map/test/model.glb#Scene0"));
        });
}

pub fn load_map_collision(){

}
