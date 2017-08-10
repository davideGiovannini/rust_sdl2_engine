use sdl2;

use gl::types::GLfloat;

use std::mem;

mod shader;
mod vertex_attrib;
pub use self::shader::*;
pub use self::vertex_attrib::*;


pub fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}


#[inline]
pub fn size_of_glfloat(num: i32) -> i32 {
    num * mem::size_of::<GLfloat>() as i32
}
