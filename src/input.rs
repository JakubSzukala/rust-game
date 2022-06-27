/* Sources: 
 * https://rust-unofficial.github.io/patterns/patterns/behavioural/command.html
 * https://doc.rust-lang.org/rust-by-example/trait/dyn.html
 * https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
 */

use bevy::{
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
    p: Query<&MovementSpeed, With<Player>>,
    ) {
    let ms = p.single(); // on fail, panics
    
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
    mut p: Query<&mut Combo, With<Player>>,
    time: Res<Time>,
    ) {
    let mut combo = p.single_mut();
    
    // tick the timer
    combo.combo_input_timer.tick(time.delta());    
    println!("Time since startup: {}", time.time_since_startup().as_secs_f64());
    if combo.combo_input_timer.finished() {
        println!("Timer finished.");
        match combo.valid_combos.get(&combo.combo_sequence) {
            Some(value) => info!("Casting a spell: {} from combo: {}.",
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


#[cfg(test)]
mod tests {
    // Use both bevy and all functions in this file 
    use bevy::prelude::*;
    use super::*;
    use std::{thread, time};
    // example test cases:
    // buffer insert if successfull DONE
    // combo picking if correct / incorrect combos
    // timeout and clearing the buffer

    #[test]
    fn check_adding_to_combo_buffer() {
        // Setup app with tested system and min plugins (necessary time res)
        let mut app = App::new();
        app
            .add_system(player_combo_input_system)
            .add_plugins(MinimalPlugins);

        // Setup the entity that will be queried
        let combo_id = app.world.spawn()
            .insert(Combo::new(1))
            .insert(Player)
            .id();

        // Setup the keyboard input resource 
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::J); 
        app.insert_resource(input);

        // Run systems
        app.update();
        
        // Lookup the combo_sequence and check if it is correct
        let res = app.world.query::<&Combo>().get(&app.world, combo_id);
        match res {
            Ok(combo) => {
                assert_eq!(combo.combo_sequence, "j");
            }
            Err(err) => {
                panic!("Combo_sequence after 1 J key press is not correct: {}.", 
                       err)
            }
        }
    }

    #[test]
    fn check_combo_buffer_handling() {
        let mut app = App::new(); 
        app
            .add_system(player_combo_input_system)
            .add_plugins(MinimalPlugins);

        let combo_id = app.world.spawn()
            .insert(Combo::new(1))
            .insert(Player)
            .id();
        
        // Initialize resources - key press and time for timeouts
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::K);
        let time = Time::default();
        app
            .insert_resource(input)
            .insert_resource(time);

        app.update();
        
        // Check single key press
        let res = app.world.query::<&Combo>().get(&app.world, combo_id);
        match res {
            Ok(combo) => assert_eq!(combo.combo_sequence, "k"),
            Err(err) => panic!("Combo sequence after 1 K key press is not correct {}", err)
        }
        
        // Clear the inputs (or they will be read again in next app.update())
        app.world.resource_mut::<Input<KeyCode>>().clear();
        
        // Check timeout reaction and if timer elapses 
        let timeout = time::Duration::from_secs(2);
        thread::sleep(timeout);
        app.update();

        let res = app.world.query::<&Combo>().get(&app.world, combo_id);
        match res {
            Ok(combo) => {
                let secs = combo.combo_input_timer.elapsed().as_secs_f64();
                assert!(secs == 1.0); // Timer freezes until reset after elapse
                assert_eq!(combo.combo_sequence, "");
            }
            Err(err) => {
                panic!("Panicked due to: {}", err.to_string())
            }
        }
    }
}










