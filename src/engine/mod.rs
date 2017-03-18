
pub mod action;
pub mod context;
pub mod game;

use sdl2;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::event::Event;
use sdl2::render::Renderer;
use std::collections::HashSet;
use sdl2::keyboard::Scancode;
use sdl2::{EventPump, TimerSubsystem};

use {EngineBuilder, EngineAction, EngineContext, Game};

use fps_counter::FpsCounter;
use game_controllers::GameControllerManager;

use super::sdl2_utils;


pub struct Engine<'window> {
    pub renderer: Renderer<'window>,
    pub ttf_context: Sdl2TtfContext,
    timer_subsystem: TimerSubsystem,
    event_pump: EventPump,
}


pub fn run_engine<G: Game>(options: &mut EngineBuilder) {
    let (width, height) = options.window_size;

    let mut engine = sdl2_utils::initialize_engine(options.window_title,
                                                width,
                                                height,
                                                options.fullscreen,
                                                options.logical_size);

    let mut fps_counter = FpsCounter::new(&mut engine.timer_subsystem);

    let mut game_controller_manager = GameControllerManager::new();

    let mut game: Box<G> = Box::new(Game::init(&engine));

    game.set_up();

    let mut keys_down: HashSet<Scancode> = Default::default();

    let clear_color = options.clear_color;

    'running: loop {

        let (should_wait, maybe_fps) = fps_counter.tick(&mut engine.timer_subsystem);
        if should_wait {
            continue;
        }
        if let Some(fps) = maybe_fps {
            let mut window = engine.renderer.window_mut().unwrap();
            let title = format!("{}: {} fps", options.window_title, fps);
            window.set_title(&title).unwrap();
        }
        // EVENT HANDLING
        for event in engine.event_pump.poll_iter() {
            use sdl2::keyboard::Keycode::*;
            match event {
                Event::Quit { .. } => break 'running,
                Event::ControllerDeviceAdded { which, .. } => {
                    game_controller_manager.added_controller(which)
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    game_controller_manager.removed_controller(which)
                }
                Event::ControllerButtonDown { which, .. } => println!("Instance id {:?}", which),
                Event::KeyDown { keycode: Some(F11), .. } => {
                    println!("Game Controllers {:#?}", game_controller_manager)
                }
                _ => {
                    game.process_event(&event);
                }
            }
        }
        // LOGIC
        let keys_snapshot = engine.event_pump.keyboard_state().pressed_scancodes().collect();
        let newly_pressed = &keys_snapshot - &keys_down;
        keys_down.clone_from(&keys_snapshot);

        let context = EngineContext::new(keys_snapshot,
                                         newly_pressed,
                                         game_controller_manager.snapshot());
        let action = game.logic(context);
        match action {
            EngineAction::Quit => break 'running,
            EngineAction::ToggleFullScreen => {
                use sdl2::video::FullscreenType;
                let mut window = engine.renderer.window_mut().unwrap();
                let status = if options.fullscreen {
                    FullscreenType::Off
                } else {
                    FullscreenType::Desktop
                };
                window.set_fullscreen(status).unwrap();
                options.fullscreen = !options.fullscreen;
            }
            _ => {}
        }

        // RENDERING
        engine.renderer.set_draw_color(clear_color);
        engine.renderer.clear();

        game.render(&mut engine);

        engine.renderer.present();
    }
    // Close up
    sdl2::mixer::close_audio();
}



pub fn make_engine<'window>(renderer: Renderer<'window>,
                            ttf_context: Sdl2TtfContext,
                            timer_subsystem: TimerSubsystem,
                            event_pump: EventPump)
                            -> Engine<'window> {
    Engine {
        renderer: renderer,
        ttf_context: ttf_context,
        timer_subsystem: timer_subsystem,
        event_pump: event_pump,
    }
}