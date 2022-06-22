use bevy::{
    prelude::*,
};

use crate::player::Player;
use crate::player::Combo;

#[derive(Component)]
pub struct UIComboDisplay;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(combo_display_setup_system);
        app.add_system(combo_display_update_system);
    }
}

fn combo_display_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
    ) {
    info!("Setting up display.");
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        style: Style {
                    align_self: AlignSelf::FlexEnd,
                    ..default()
                },
                // Use `Text` directly
                text: Text {
                    // Construct a `Vec` of `TextSection`s
                    sections: vec![
                        TextSection {
                            value: "Combo: ".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 200.0,
                                color: Color::RED,
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 200.0,
                                color: Color::RED,
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 200.0,
                                color: Color::RED,
                            },
                        }
                    ],
                    ..default()
                },
                ..default()
            })
            .insert(UIComboDisplay);
}

// Can we do it with one query?
fn combo_display_update_system(
    mut q1: Query<&mut Text, With<UIComboDisplay>>,
    q2: Query<&Combo, With<Player>>
    ) {
    let mut combo_display = q1.single_mut();
    let combo = q2.single();
    combo_display.sections[1].value = String::from(&combo.combo_sequence);
}












