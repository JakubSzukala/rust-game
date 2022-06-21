/* Sources: 
 * https://rust-unofficial.github.io/patterns/patterns/behavioural/command.html
 * https://doc.rust-lang.org/rust-by-example/trait/dyn.html
 * https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
 */

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use crate::player::Player;
use crate::player::MovementSpeed;
use crate::player::Combo;

pub struct InputHandlerPlugin;
impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App){
        info!("Hi from app, adding a keyboard_input_system");
        app.add_system(player_mv_input_system);
        app.add_system(player_combo_input_system);
    }
}

// Probably will have to be split into multiple systems for handling 
// Player, Player + envrinoment etc?
// Or "interactable" obj should have a mark or smth?
// TODO: split this
fn player_mv_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    p: Query<(&Player, &MovementSpeed)>,
    ) {
    let (_, ms) = p.single(); // on fail, panics
    
    // Movement 
    if keyboard_input.pressed(KeyCode::W) {
        info!("Pressed W -> Walking forward with v = {}", 
              ms.0);
    }

    if keyboard_input.pressed(KeyCode::S) {
        info!("Pressed S -> Walking backward with v = {}",
              ms.0);
    }
    
    if keyboard_input.pressed(KeyCode::A) {
        info!("Pressed A -> Walking left with v = {}", 
              ms.0);
    }

    if keyboard_input.pressed(KeyCode::D) {
        info!("Pressed D -> Walking right with v = {}",
              ms.0);
    }
}


fn player_combo_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut p: Query<(&Player, &mut Combo)>,
    time: Res<Time>,
    ) {
    let (_, mut combo) = p.single_mut();
    
    // tick the timer
    combo.combo_input_timer.tick(time.delta());    

    if combo.combo_input_timer.finished() {
        match combo.valid_combos.get(&combo.combo_sequence) {
            Some(value) => info!("Casting a spel: {} from combo: {}.",
                                 value, combo.combo_sequence),
            None        => ()
        };
        combo.combo_sequence = String::from("");
    }

    if keyboard_input.just_pressed(KeyCode::J) {
        info!("Registered J press.");
        combo.combo_sequence.push('j');
        combo.combo_input_timer.reset();
    }
    
    if keyboard_input.just_pressed(KeyCode::K) {
        info!("Registered K press.");
        combo.combo_sequence.push('k');
        combo.combo_input_timer.reset();
    }
}













