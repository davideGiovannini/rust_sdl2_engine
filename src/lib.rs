extern crate gl;
pub extern crate sdl2;

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
pub use engine::game::{AnyGameScene, FromEngine, GameScene};
pub use game_controllers::{GameController, GameControllerManager};

pub use engine::context::EngineContext;
pub use engine::action::EngineAction;
pub use engine::Engine;

pub use sdl2_utils::log_system_info;

pub mod resources;

pub mod font;

const WINDOW_SIZE: (u32, u32) = (800, 600);
const CLEAR_COLOR: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub struct EngineBuilder<'window> {
    window_title: &'window str,
    window_size: (u32, u32),
    logical_size: Option<(u32, u32)>,
    fullscreen: bool,
    clear_color: Color,
}

impl Engine {
    pub fn new(window_title: &str) -> EngineBuilder {
        EngineBuilder {
            window_title,
            window_size: WINDOW_SIZE,
            logical_size: None,
            clear_color: CLEAR_COLOR,
            fullscreen: false,
        }
    }
}

impl<'window> EngineBuilder<'window> {
    /// Set the initial size of the window.
    pub fn with_window_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_size = (width, height);
        self
    }

    /// Set the logical render size.
    pub fn with_logical_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.logical_size = Some((width, height));
        self
    }

    pub fn with_clear_color(&mut self, color: Color) -> &mut Self {
        self.clear_color = color;
        self
    }

    pub fn with_fullscreen(&mut self, fullscreen: bool) -> &mut Self {
        self.fullscreen = fullscreen;
        self
    }

    /// Start the engine.
    pub fn start<Scene: 'static>(&mut self)
    where
        Scene: GameScene + FromEngine,
    {
        engine::run_engine::<Scene>(self)
    }
}

// RE-EXPORTS

pub mod keyboard {
    pub use sdl2::keyboard::Scancode;
}
