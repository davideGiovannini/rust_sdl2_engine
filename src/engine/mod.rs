
pub mod action;
pub mod context;
pub mod game;

use sdl2;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::event::Event;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashSet;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;

use {EngineBuilder, EngineAction, EngineContext, AnyGameScene, GameScene, FromEngine};

use fps_counter::FpsCounter;
use game_controllers::GameControllerManager;

use super::sdl2_utils;


pub struct Engine {
    pub renderer: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>,
    pub ttf_context: Sdl2TtfContext,
    event_pump: EventPump,
}


pub fn run_engine<Scene: 'static>(options: &mut EngineBuilder) where Scene: GameScene+FromEngine {
    let (width, height) = options.window_size;

    let mut engine = sdl2_utils::initialize_engine(options.window_title,
                                                width,
                                                height,
                                                options.fullscreen,
                                                options.logical_size);

    let mut fps_counter = FpsCounter::new();

    let mut game_controller_manager = GameControllerManager::new();

    let mut game: AnyGameScene = Box::new(Scene::init(&engine));

    game.set_up();

    let mut game_stack = vec![game];

    let mut keys_down: HashSet<Scancode> = Default::default();

    let clear_color = options.clear_color;

    'running: loop {

        let (should_wait, maybe_fps, delta_time) = fps_counter.tick();
        if should_wait {
            continue;
        }
        if let Some(fps) = maybe_fps {
            let window = engine.renderer.window_mut();
            let title = format!("{}: {} fps", options.window_title, fps);
            window.set_title(&title).unwrap();
        }
        // EVENT HANDLING
        for event in engine.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::ControllerDeviceAdded { which, .. } => {
                    game_controller_manager.added_controller(which)
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    game_controller_manager.removed_controller(which)
                }
                _ => {
                    game_stack.last_mut().unwrap().process_event(&event);
                }
            }
        }
        // LOGIC
        let keys_snapshot = engine.event_pump.keyboard_state().pressed_scancodes().collect();
        let newly_pressed = &keys_snapshot - &keys_down;
        keys_down.clone_from(&keys_snapshot);

        let context = EngineContext::new(keys_snapshot,
                                         newly_pressed,
                                         delta_time,
                                         game_controller_manager.snapshot());
        let action = game_stack.last_mut().unwrap().logic(context);
        match action {
            EngineAction::Quit => break 'running,
            EngineAction::ToggleFullScreen => {
                use sdl2::video::FullscreenType;
                let window = engine.renderer.window_mut();
                let status = if options.fullscreen {
                    FullscreenType::Off
                } else {
                    FullscreenType::Desktop
                };
                window.set_fullscreen(status).unwrap();
                options.fullscreen = !options.fullscreen;
            }
            EngineAction::PopScene => {
                    drop(game_stack.pop());
                    if let Some(scene) = game_stack.last_mut(){
                        scene.on_resume();
                        continue 'running
                    }else{
                        break 'running
                    }
            }
            EngineAction::PushScene(mut get_scene) => {
                game_stack.last_mut().unwrap().on_pause();
                let mut next_scene = get_scene(&engine);
                next_scene.set_up();
                game_stack.push(next_scene);
                continue 'running
            }
            EngineAction::SwitchToScene(mut get_scene) => {
                drop(game_stack.pop());
                let mut next_scene = get_scene(&engine);
                next_scene.set_up();
                game_stack.push(next_scene);
                continue 'running
            }
            _ => {}
        }

        // RENDERING
        engine.renderer.set_draw_color(clear_color);
        engine.renderer.clear();

        game_stack.last_mut().unwrap().render(&mut engine);

        engine.renderer.present();
    }
    // Close up
    sdl2::mixer::close_audio();
}



pub fn make_engine(renderer: WindowCanvas,
                   texture_creator: TextureCreator<WindowContext>,
                   ttf_context: Sdl2TtfContext,
                   event_pump: EventPump)
            -> Engine {
    Engine {
        renderer,
        texture_creator,
        ttf_context,
        event_pump,
    }
}
