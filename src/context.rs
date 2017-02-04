use sdl2::keyboard::Scancode;
use std::collections::HashSet;

#[derive(Clone)]
pub struct EngineContext {
    keyboard_down: HashSet<Scancode>,
    keyboard_pressed: HashSet<Scancode>,
}

impl EngineContext {
    pub fn new(keys_down: HashSet<Scancode>, keys_pressed: HashSet<Scancode>) -> EngineContext {
        EngineContext {
            keyboard_down: keys_down,
            keyboard_pressed: keys_pressed,
        }
    }
    pub fn is_key_down(&self, scancode: Scancode) -> bool {
        self.keyboard_down.contains(&scancode)
    }
    pub fn is_key_pressed(&self, scancode: Scancode) -> bool {
        self.keyboard_pressed.contains(&scancode)
    }
}
