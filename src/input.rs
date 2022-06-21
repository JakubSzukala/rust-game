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

pub struct InputHandlerPlugin;
impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App){
        info!("Hi from app, adding a keyboard_input_system");
        app.add_system(keyboard_player_input_system);
    }
}

// Probably will have to be split into multiple systems for handling 
// Player, Player + envrinoment etc?
// Or "interactable" obj should have a mark or smth?
fn keyboard_player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    p: Query<(&Player, &MovementSpeed)>) {
    let (_, ms) = p.single(); // on fail, panics
    
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
    // etc
}


