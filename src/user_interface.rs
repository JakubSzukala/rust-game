use bevy::prelude::*;

use crate::player::Combo;
use crate::player::Player;

#[derive(Component)]
pub struct UIComboDisplay;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_general_setup);
        app.add_startup_system(combo_display_setup_system);
        app.add_system(combo_display_update_system);
    }
}

fn ui_general_setup(mut commands: Commands) {
    info!("Setting up display.");
    commands.spawn_bundle(UiCameraBundle::default());
}

fn combo_display_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    // Border and Combo Display as a child
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(window.width()), Val::Px(200.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                border: Rect::all(Val::Px(20.0)),
                ..default()
            },
            color: UiColor(Color::rgba(0.0, 0.0, 1.0, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            ..default()
                        },
                        border: Rect::all(Val::Px(20.0)),
                        ..default()
                    },
                    text: Text {
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                        // If more sections are added, there is some problem with
                        // positioning of more than 1 section ;_;
                        sections: vec![TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 200.0,
                                color: Color::SEA_GREEN,
                            },
                        }],
                        ..default()
                    },
                    ..default()
                })
                .insert(UIComboDisplay);
        });
}

// Can we do it with one query?
fn combo_display_update_system(
    mut q1: Query<&mut Text, With<UIComboDisplay>>,
    q2: Query<&Combo, With<Player>>,
) {
    let mut combo_display = q1.single_mut();
    let combo = q2.single();
    combo_display.sections[0].value = String::from(&combo.combo_sequence);
}
