use sdl2::keyboard::Scancode;
use std::collections::{HashMap, HashSet};

use game_controllers::GameController;

#[derive(Clone, Default)]
pub struct EngineContext {
    keyboard_down: HashSet<Scancode>,
    keyboard_pressed: HashSet<Scancode>,
    pub delta_time: u32,
    pub controllers: HashMap<i32, GameController>,
}

impl EngineContext {
    pub fn new(keyboard_down: HashSet<Scancode>,
               keyboard_pressed: HashSet<Scancode>,
               delta_time: u32,
               controllers: HashMap<i32, GameController>)
               -> EngineContext {
        EngineContext {
            keyboard_down,
            keyboard_pressed,
            delta_time,
            controllers,
        }
    }
    pub fn is_key_down(&self, scancode: Scancode) -> bool {
        self.keyboard_down.contains(&scancode)
    }
    pub fn is_key_pressed(&self, scancode: Scancode) -> bool {
        self.keyboard_pressed.contains(&scancode)
    }
}
