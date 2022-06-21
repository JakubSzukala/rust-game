/* Sources: 
 * https://rust-unofficial.github.io/patterns/patterns/behavioural/command.html
 * https://doc.rust-lang.org/rust-by-example/trait/dyn.html
 * https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
 */

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

pub struct InputHandlerPlugin;
impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App){
        info!("Hi from app, adding a keyboard_input_system");
        app.add_system(keyboard_input_system);
    }
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::W) {
        info!("Pressed W -> Walking forward.");
    }

    if keyboard_input.pressed(KeyCode::S) {
        info!("Pressed S -> Walking backward.");
    }
    
    if keyboard_input.pressed(KeyCode::A) {
        info!("Pressed A -> Walking left.");
    }

    if keyboard_input.pressed(KeyCode::D) {
        info!("Pressed D -> Walking right.");
    }
    // etc
}



