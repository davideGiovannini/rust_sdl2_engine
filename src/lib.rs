extern crate sdl2;

use sdl2::image::INIT_PNG; // INIT_JPG

use sdl2::pixels::Color;
use sdl2::event::Event;

use sdl2::mixer::INIT_OGG;
use std::collections::HashSet;
use sdl2::keyboard::Scancode;

mod fps_counter;
mod context;
mod game;
pub mod math;
pub use game::Game;

pub use self::context::EngineContext;
use self::fps_counter::FpsCounter;


pub const LOGICAL_SIZE: (u32, u32) = (420, 340);

pub struct Engine<'window, G: Game> {
    window_title: &'window str,
    timer_subsystem: sdl2::TimerSubsystem,
    renderer: sdl2::render::Renderer<'window>,
    event_pump: sdl2::EventPump,
    fps_counter: FpsCounter,
    game: Box<G>,
}


impl<'window, G: Game> Engine<'window, G> {
    pub fn new(width: u32, height: u32, window_title: &str) -> Engine<G> {
        let sdl_context = sdl2::init().unwrap();
        let _image_context = sdl2::image::init(INIT_PNG).unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();
        let mut timer = sdl_context.timer().unwrap();

        let _mixer_context = sdl2::mixer::init(INIT_OGG);
        sdl2::mixer::open_audio(44100, sdl2::mixer::AUDIO_S16LSB, 2, 1024).unwrap();
        sdl2::mixer::allocate_channels(32);

        let window = video_subsystem.window(window_title, width, height)
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


        // TODO improve
        let (width, height) = LOGICAL_SIZE;
        renderer.set_logical_size(width, height).unwrap();


        let event_pump = sdl_context.event_pump().unwrap();
        let fps_counter = FpsCounter::new(&mut timer);
        let game = Box::new(Game::init(&renderer, &ttf_context));

        Engine {
            window_title: window_title,
            timer_subsystem: timer,
            renderer: renderer,
            event_pump: event_pump,
            fps_counter: fps_counter,
            game: game,
        }
    }


    pub fn game_loop(&mut self) {
        self.game.set_up();


        let mut keys_down: HashSet<Scancode> = Default::default();

        'running: loop {

            let (should_wait, maybe_fps) = self.fps_counter.tick(&mut self.timer_subsystem);
            if should_wait {
                continue;
            }
            if let Some(fps) = maybe_fps {
                let mut window = self.renderer.window_mut().unwrap();
                let title = format!("{}: {} fps", self.window_title, fps);
                window.set_title(&title).unwrap();
            }
            // EVENT HANDLING

            for event in self.event_pump.poll_iter() {
                use sdl2::keyboard::Keycode::*;
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { keycode: Some(Escape), .. } => break 'running,
                    _ => {}
                }
            }
            // LOGIC
            let keys_snapshot = self.event_pump
                .keyboard_state()
                .pressed_scancodes()
                .collect();
            let newly_pressed = &keys_snapshot - &keys_down;
            keys_down.clone_from(&keys_snapshot);

            let context = EngineContext::new(keys_snapshot, newly_pressed);
            self.game.logic(context);

            // RENDERING
            self.renderer.set_draw_color(Color::RGB(0, 0, 0));
            self.renderer.clear();


            self.game.render(&mut self.renderer);

            self.renderer.present();
        }
    }
}

impl <'window, G: Game>Drop for Engine<'window, G>{
    fn drop(&mut self){
        sdl2::mixer::close_audio();
    }
}
