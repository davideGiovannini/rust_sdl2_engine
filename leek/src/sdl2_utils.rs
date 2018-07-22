use sdl2;
use sdl2::image::INIT_PNG; // INIT_JPG

use opengl;
use gl;

use engine::make_engine;
use Engine;

use failure::{err_msg, Error};

#[inline]
pub fn initialize_engine(
    window_title: &str,
    width: u32,
    height: u32,
    fullscreen: bool,
    logical_size: Option<(u32, u32)>,
) -> Result<Engine, Error> {
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG).unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut window_builder = video_subsystem.window(window_title, width, height);
    window_builder.position_centered().opengl().resizable();

    if fullscreen {
        window_builder.fullscreen_desktop();
    }

    let window = window_builder.build()?;

    let mut renderer = window
        .into_canvas()
        .accelerated()
        .index(opengl::find_sdl_gl_driver().ok_or_else(|| err_msg("Could not find sdl gl driver"))?)
        .target_texture()
        .build()
        .unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_framebuffer_srgb_compatible(true);
    gl_attr.set_stencil_size(8);
    gl_attr.set_depth_size(24);

    // Set the context into debug mode
    #[cfg(debug_assertions)]
    gl_attr.set_context_flags().debug().set();
    // Set the OpenGL context version (OpenGL 3.1)
    gl_attr.set_context_version(3, 1);

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    renderer
        .window()
        .gl_set_context_to_current()
        .map_err(err_msg)?;

    #[cfg(debug_assertions)]
    println!(
        "Opengl {}.{} version {:#?}",
        gl_attr.context_major_version(),
        gl_attr.context_minor_version(),
        gl_attr.context_profile()
    );

    if let Some((width, height)) = logical_size {
        renderer.set_logical_size(width, height).map_err(err_msg)?;
    }

    let event_pump = sdl_context.event_pump().map_err(err_msg)?;

    let texture_creator = renderer.texture_creator();

    make_engine(
        sdl_context,
        renderer,
        texture_creator,
        ttf_context,
        event_pump,
    )
}

pub fn log_system_info() -> String {
    format!(
        r#"System info:
  SDL2 [{}]
  SDL2 image [{}]
  SDL2 ttf [{}]"#,
        sdl2::version::version(),
        sdl2::image::get_linked_version(),
        sdl2::ttf::get_linked_version()
    )
}
