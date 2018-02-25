pub mod action;
pub mod context;
pub mod game;

use alto;

use failure::{err_msg, Error};

use sdl2::ttf::Sdl2TtfContext;
use sdl2::event::Event;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::collections::HashSet;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use sdl2::pixels::Color;

use {AnyGameScene, EngineAction, EngineBuilder, EngineContext, FromEngine, GameScene};

use fps_counter::FpsCounter;
use game_controllers::GameControllerManager;

use super::resources_cache::Resources;

use super::sdl2_utils;

use imgui::ImGui;
use imgui_backend;
use opengl::log_messages;

pub struct Engine {
    pub renderer: WindowCanvas,
    pub ttf_context: Sdl2TtfContext,
    pub resources: Resources,
    pub alto_context: alto::Context,
    pub clear_color: Color,
    event_pump: EventPump,
}

pub fn run_engine<Scene: 'static>(options: &mut EngineBuilder) -> Result<(), Error>
where
    Scene: GameScene + FromEngine,
{
    let (width, height) = options.window_size;

    let mut engine = sdl2_utils::initialize_engine(
        options.window_title,
        width,
        height,
        options.fullscreen,
        options.logical_size,
    )?;

    let mut imgui = ImGui::init();
    imgui_backend::configure_keys(&mut imgui);
    imgui.set_font_global_scale(options.imgui_font_scale);

    let mut imgui_renderer =
        imgui_backend::Renderer::init(&mut imgui).expect("Failed to initialize imgui_renderer");

    let mut fps_counter = FpsCounter::new();

    let mut game_controller_manager = GameControllerManager::new();

    let mut game: AnyGameScene = Box::new(Scene::init(&mut engine));

    game.set_up();

    let mut game_stack = vec![game];

    let mut keys_down: HashSet<Scancode> = Default::default();

    engine.clear_color = options.clear_color;

    'running: loop {
        let (should_wait, maybe_fps, delta_time) = fps_counter.tick();
        if should_wait {
            continue;
        }
        if let Some(fps) = maybe_fps {
            let window = engine.renderer.window_mut();
            let title = format!("{}: {} fps", options.window_title, fps);
            window.set_title(&title)?;
        }

        // EVENT HANDLING
        for event in engine.event_pump.poll_iter() {
            imgui_backend::process_event(&mut imgui, &event);

            match event {
                Event::Quit { .. } => break 'running,
                Event::ControllerDeviceAdded { which, .. } => {
                    game_controller_manager.added_controller(which)
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    // TODO check if cast is ok
                    game_controller_manager.removed_controller(which as u32)
                }
                _ => {
                    game_stack.last_mut().unwrap().process_event(&event);
                }
            }
        }

        imgui_backend::process_event_state(&mut imgui, &engine.event_pump);

        let size_points = engine.renderer.window().size();
        let size_pixels = engine.renderer.window().drawable_size();
        let ui = imgui.frame(size_points, size_pixels, 0.016);

        // LOGIC
        let keys_snapshot = engine
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .collect();
        let newly_pressed = &keys_snapshot - &keys_down;
        keys_down.clone_from(&keys_snapshot);

        {
            let context = EngineContext::new(
                keys_snapshot,
                newly_pressed,
                delta_time,
                fps_counter.elapsed(),
                game_controller_manager.snapshot(),
            );
            let action = game_stack.last_mut().unwrap().logic(&context, &engine, &ui);
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
                    window.set_fullscreen(status).map_err(err_msg)?;
                    options.fullscreen = !options.fullscreen;
                }
                EngineAction::PopScene => {
                    drop(game_stack.pop());
                    if let Some(scene) = game_stack.last_mut() {
                        scene.on_resume();
                        continue 'running;
                    } else {
                        break 'running;
                    }
                }
                EngineAction::PushScene(mut get_scene) => {
                    game_stack.last_mut().unwrap().on_pause();
                    let mut next_scene = get_scene(&mut engine);
                    next_scene.set_up();
                    game_stack.push(next_scene);
                    continue 'running;
                }
                EngineAction::SwitchToScene(mut get_scene) => {
                    drop(game_stack.pop());
                    let mut next_scene = get_scene(&mut engine);
                    next_scene.set_up();
                    game_stack.push(next_scene);
                    continue 'running;
                }
                _ => {}
            }

            // RENDERING
            engine.renderer.set_draw_color(engine.clear_color);
            engine.renderer.clear();

            game_stack.last_mut().unwrap().render(&context, &mut engine);
        }
        imgui_renderer.render(ui).unwrap();

        engine.renderer.present();

        #[cfg(debug_assertions)]
        log_messages();
    }
    // Close up
    Ok(())
}

pub fn make_engine(
    renderer: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: Sdl2TtfContext,
    event_pump: EventPump,
) -> Result<Engine, Error> {
    let alto_context = super::alto_utils::initialize_context()?;

    Ok(Engine {
        renderer,
        ttf_context,
        event_pump,
        alto_context: alto_context.clone(),
        clear_color: Color::RGB(0,0,0),
        resources: Resources::new(texture_creator, alto_context.clone()),
    })
}
