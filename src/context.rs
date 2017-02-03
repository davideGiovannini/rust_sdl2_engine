use sdl2::keyboard::Scancode;
use std::collections::HashSet;

#[derive(Clone)]
pub struct EngineContext {
    pub keyboard_pressed: HashSet<Scancode>,
}
