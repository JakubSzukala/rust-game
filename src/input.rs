
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

// TODO: change it into a enum?

pub struct AgentStub {
    x: u32,
    y: u32,
    lvl: u8,
    speed: u8
}

impl AgentStub {
    fn say_hi(hi_str: String) {
        println!("Walked forward distance {}", hi_str);
    }
}


pub trait Command {
    fn execute(&self);
}

struct WalkForwardCommand {agent: AgentStub}
struct WalkBackwardCommand {agent: AgentStub}
struct WalkLeftCommand {agent: AgentStub}
struct WalkRightCommand {agent: AgentStub}
struct DoNothingCommand {agent: AgentStub}

impl Command for WalkForwardCommand {
    fn execute(&self) {
        println!("Walking forward with v = {}", self.agent.speed)
    }
}
impl Command for WalkBackwardCommand {
    fn execute(&self) {
        println!("Walking backward with v = {}", self.agent.speed)
    }
}
impl Command for WalkLeftCommand {
    fn execute(&self) {
        println!("Walking left with v = {}", self.agent.speed)
    }
} 
impl Command for WalkRightCommand {
    fn execute(&self) {
        println!("Walking right with v = {}", self.agent.speed)
    }
}
impl Command for DoNothingCommand {
    fn execute(&self) {
        println!("Doing nothing actually :)");
    }
}


pub struct InputHandler {
    agent: AgentStub
}


// TODO: change hard coding (WSAD etc) to some intialization 
impl InputHandler {
    pub fn handle_inputs(&self, keyboard_input: Res<Input<KeyCode>>) 
        -> Box<dyn Command> {
        if keyboard_input.pressed(KeyCode::W) {
            Box::new(WalkForwardCommand {agent: self.agent})
        } 
        else if keyboard_input.pressed(KeyCode::S) {
            Box::new(WalkBackwardCommand {agent: self.agent})
        } 
        else if keyboard_input.pressed(KeyCode::A) {
            Box::new(WalkLeftCommand {agent: self.agent}) 
        } 
        else if keyboard_input.pressed(KeyCode::A) {
            Box::new(WalkRightCommand {agent: self.agent}) 
        } 
        else {
            // Box::new allocates mem on heap, isnt it actually leak XD???
            // Hopefully not considering rust scoping rules
            Box::new(DoNothingCommand {agent: self.agent}) 
        }
    }
}



