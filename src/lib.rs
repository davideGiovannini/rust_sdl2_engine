extern crate sdl2;
extern crate sdl2_sys;
extern crate gl;

use sdl2::pixels::Color;

mod engine;
mod fps_counter;
mod game_controllers;
mod opengl;
mod post_processing;
mod sdl2_utils;

#[macro_use]
mod common_macros;

pub use post_processing::PostProcessEffect as PostProcessingEffect;

pub mod math;
pub use engine::game::{AnyGameScene, GameScene};
pub use game_controllers::{GameControllerManager, GameController};

pub use engine::context::EngineContext;
pub use engine::action::EngineAction;
pub use engine::Engine;

pub use sdl2_utils::log_system_info;

const WINDOW_SIZE: (u32, u32) = (800, 600);
const CLEAR_COLOR: Color = Color::RGB(0, 0, 0);

pub struct EngineBuilder<'window> {
    window_title: &'window str,
    window_size: (u32, u32),
    logical_size: Option<(u32, u32)>,
    fullscreen: bool,
    clear_color: Color,
}

impl <'window>Engine<'window>{
    pub fn new(window_title: &str) -> EngineBuilder {
        EngineBuilder {
            window_title: window_title,
            window_size: WINDOW_SIZE,
            logical_size: None,
            clear_color: CLEAR_COLOR,
            fullscreen: false
        }
    }
}

impl<'window> EngineBuilder<'window> {

    pub fn with_window_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_size = (width, height);
        self
    }

    pub fn with_logical_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.logical_size = Some((width, height));
        self
    }

    pub fn with_clear_color(&mut self, color: Color) -> &mut Self{
        self.clear_color = color;
        self
    }

    pub fn with_fullscreen(&mut self, fullscreen: bool) -> &mut Self{
        self.fullscreen = fullscreen;
        self
    }

    pub fn start(&mut self, initial_scene: fn(&Engine) -> AnyGameScene) {
        engine::run_engine(self, initial_scene)
    }
}
