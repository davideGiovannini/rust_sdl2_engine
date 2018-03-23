extern crate gl;
pub extern crate sdl2;

pub extern crate alto;
extern crate failure;
pub extern crate lewton;
extern crate notify;

#[macro_use]
pub extern crate imgui;

use sdl2::pixels::Color;

mod engine;
mod fps_counter;
mod game_controllers;
mod opengl;
mod imgui_backend;
mod post_processing;
mod sdl2_utils;
pub mod alto_utils;
pub mod prelude;

pub mod resources;
mod debug;

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

pub mod font;
pub mod assets;

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
    hide_cursor: bool,
    relative_cursor: bool,
    clear_color: Color,
    imgui_font_scale: f32,
}

impl Engine {
    pub fn new(window_title: &str) -> EngineBuilder {
        EngineBuilder {
            window_title,
            window_size: WINDOW_SIZE,
            logical_size: None,
            clear_color: CLEAR_COLOR,
            fullscreen: false,
            hide_cursor: false,
            relative_cursor: false,
            imgui_font_scale: 1.5,
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

    pub fn with_imgui_font_scale(&mut self, font_scale: f32) -> &mut Self {
        self.imgui_font_scale = font_scale;
        self
    }

    pub fn with_hidden_cursor(&mut self, hide_cursor: bool) -> &mut Self {
        self.hide_cursor = hide_cursor;
        self
    }

    pub fn with_relative_cursor(&mut self, relative_cursor: bool) -> &mut Self {
        self.relative_cursor = relative_cursor;
        self
    }

    /// Start the engine.
    pub fn start<Scene: 'static>(&mut self)
    where
        Scene: GameScene + FromEngine,
    {
        if let Err(error) = engine::run_engine::<Scene>(self) {
            println!("{:?}", error)
        }
    }
}

// RE-EXPORTS

pub mod keyboard {
    pub use sdl2::keyboard::Scancode;
}
