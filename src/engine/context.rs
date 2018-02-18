use sdl2::keyboard::Scancode;
use std::collections::{HashMap, HashSet};

use game_controllers::GameController;

use imgui::Ui;

#[derive(Clone)]
pub struct EngineContext<'frame> {
    keyboard_down: HashSet<Scancode>,
    keyboard_pressed: HashSet<Scancode>,
    pub delta_time: u32,
    pub elapsed_time: u64,
    // TODO use a reference instead of a clone
    pub controllers: HashMap<u32, GameController>,
    pub ui: &'frame Ui<'frame>,
}

impl<'frame> EngineContext<'frame> {
    pub fn new(
        keyboard_down: HashSet<Scancode>,
        keyboard_pressed: HashSet<Scancode>,
        delta_time: u32,
        elapsed_time: u64,
        controllers: HashMap<u32, GameController>,
        ui: &'frame Ui<'frame>,
    ) -> EngineContext<'frame> {
        EngineContext {
            keyboard_down,
            keyboard_pressed,
            delta_time,
            elapsed_time,
            controllers,
            ui,
        }
    }
    pub fn is_key_down(&self, scancode: Scancode) -> bool {
        self.keyboard_down.contains(&scancode)
    }
    pub fn is_key_pressed(&self, scancode: Scancode) -> bool {
        self.keyboard_pressed.contains(&scancode)
    }
}
