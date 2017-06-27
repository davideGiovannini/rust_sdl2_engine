use sdl2;
use sdl2::image::INIT_PNG; // INIT_JPG

use sdl2::mixer::INIT_OGG;

use opengl;
use gl;

use engine::make_engine;
use Engine;

#[inline]
pub fn initialize_engine(window_title: &str,
                         width: u32,
                         height: u32,
                         fullscreen: bool,
                         logical_size: Option<(u32, u32)>)
                         -> Engine {
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG).unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let _mixer_context = init_sdl_mixer();


    let mut window_builder = video_subsystem.window(window_title, width, height);
    window_builder.position_centered().opengl().resizable();

    if fullscreen {
        window_builder.fullscreen_desktop();
    }

    let window = window_builder.build().unwrap();

    let mut renderer = window.into_canvas()
        .accelerated()
        .index(opengl::find_sdl_gl_driver().unwrap())
        .target_texture()
        .build()
        .unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    // Set the context into debug mode
    #[cfg(debug_assertions)]
    gl_attr.set_context_flags().debug().set();
    // Set the OpenGL context version (OpenGL 3.1)
    gl_attr.set_context_version(3, 1);


    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    renderer.window()
        .gl_set_context_to_current()
        .unwrap();

    #[cfg(debug_assertions)]
    println!("Opengl {}.{} version {:#?}",
             gl_attr.context_major_version(),
             gl_attr.context_minor_version(),
             gl_attr.context_profile());

    if let Some((width, height)) = logical_size {
        renderer.set_logical_size(width, height).unwrap();
    }

    let event_pump = sdl_context.event_pump().unwrap();

    make_engine(renderer, ttf_context, timer_subsystem, event_pump)
}






#[inline]
fn init_sdl_mixer() -> sdl2::mixer::Sdl2MixerContext {
    let _mixer_context = sdl2::mixer::init(INIT_OGG).unwrap();
    sdl2::mixer::open_audio(44100, sdl2::mixer::AUDIO_S16LSB, 2, 1024).unwrap();
    sdl2::mixer::allocate_channels(32);
    _mixer_context
}


pub fn log_system_info() -> String {
    format!(r#"System info:
  SDL2 [{}]
  SDL2 image [{}]
  SDL2 mixer [{}]
  SDL2 ttf [{}]"#,
            sdl2::version::version(),
            sdl2::image::get_linked_version(),
            sdl2::mixer::get_linked_version(),
            sdl2::ttf::get_linked_version())
}
