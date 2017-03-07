extern crate sdl2;
extern crate sdl2_sys;
extern crate specs;

use sdl2::image::INIT_PNG; // INIT_JPG

use sdl2::pixels::Color;
use sdl2::event::Event;

use sdl2::mixer::INIT_OGG;
use std::collections::HashSet;
use sdl2::keyboard::Scancode;

mod fps_counter;
mod context;
mod game;
mod game_controllers;

#[macro_use]
mod common_macros;

pub mod math;
pub use game::Game;
pub use game_controllers::{GameControllerManager, GameController};

pub use self::context::EngineContext;
use self::fps_counter::FpsCounter;

pub const WINDOW_SIZE: (u32, u32) = (800, 600);
pub const CLEAR_COLOR: Color = Color::RGB(0, 0, 0);

pub struct Engine<'window> {
    window_title: &'window str,
    window_size: Option<(u32, u32)>,
    logical_size: Option<(u32, u32)>,
    clear_color: Option<Color>,
}


impl<'window> Engine<'window> {
    pub fn new(window_title: &str) -> Engine {
        Engine {
            window_title: window_title,
            window_size: None,
            logical_size: None,
            clear_color: None,
        }
    }

    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_size = Some((width, height));
        self
    }

    pub fn with_logical_size(mut self, width: u32, height: u32) -> Self {
        self.logical_size = Some((width, height));
        self
    }

    pub fn with_clear_color(mut self, color: Color) -> Self{
        self.clear_color = Some(color);
        self
    }

    pub fn start<G: Game>(&mut self) {
        let (width, height) = self.window_size.unwrap_or(WINDOW_SIZE);


        let sdl_context = sdl2::init().unwrap();
        let _image_context = sdl2::image::init(INIT_PNG).unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();
        let mut timer_subsystem = sdl_context.timer().unwrap();

        let _mixer_context = Engine::init_sdl_mixer();

        let window = video_subsystem.window(self.window_title, width, height)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .unwrap();


        let mut renderer = window.renderer()
            .accelerated()
            .target_texture()
            .build()
            .unwrap();


        if let Some((width, height)) = self.logical_size {
            renderer.set_logical_size(width, height).unwrap();
        }

        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut fps_counter = FpsCounter::new(&mut timer_subsystem);


        let mut game_controller_manager = GameControllerManager::new();


        let mut game: Box<G> = Box::new(Game::init(&renderer, ttf_context));

        game.set_up();

        let mut keys_down: HashSet<Scancode> = Default::default();

        let clear_color = self.clear_color.unwrap_or(CLEAR_COLOR);

        'running: loop {

            let (should_wait, maybe_fps) = fps_counter.tick(&mut timer_subsystem);
            if should_wait {
                continue;
            }
            if let Some(fps) = maybe_fps {
                let mut window = renderer.window_mut().unwrap();
                let title = format!("{}: {} fps", self.window_title, fps);
                window.set_title(&title).unwrap();
            }
            // EVENT HANDLING
            for event in event_pump.poll_iter() {
                use sdl2::keyboard::Keycode::*;
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::ControllerDeviceAdded { which, .. } => {
                        game_controller_manager.added_controller(which)
                    }
                    Event::ControllerDeviceRemoved { which, .. } => {
                        game_controller_manager.removed_controller(which)
                    }
                    Event::ControllerButtonDown { which, .. } => {
                        println!("Instance id {:?}", which)
                    }
                    Event::KeyDown { keycode: Some(Escape), .. } => break 'running,
                    Event::KeyDown { keycode: Some(F11), .. } => {
                        println!("Game Controllers {:#?}", game_controller_manager)
                    }
                    _ => {}
                }
            }
            // LOGIC
            let keys_snapshot = event_pump.keyboard_state()
                .pressed_scancodes()
                .collect();
            let newly_pressed = &keys_snapshot - &keys_down;
            keys_down.clone_from(&keys_snapshot);

            let context = EngineContext::new(keys_snapshot,
                                             newly_pressed,
                                             game_controller_manager.snapshot());
            game.logic(context);

            // RENDERING
            renderer.set_draw_color(clear_color);
            renderer.clear();

            game.render(&mut renderer);

            renderer.present();
        }
        // Close up
        sdl2::mixer::close_audio();
    }

    #[inline]
    fn init_sdl_mixer() -> sdl2::mixer::Sdl2MixerContext {
        let _mixer_context = sdl2::mixer::init(INIT_OGG).unwrap();
        sdl2::mixer::open_audio(44100, sdl2::mixer::AUDIO_S16LSB, 2, 1024).unwrap();
        sdl2::mixer::allocate_channels(32);
        _mixer_context
    }
}
