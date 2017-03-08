use sdl2;

use gl;
use gl::types::*;

use std::{ffi, mem, ptr};
use std::os::raw::c_void;

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
    return num * mem::size_of::<GLfloat>() as i32;
}

pub unsafe fn compile_program(vertex_shader: GLuint,
                              fragment_shader: GLuint)
                              -> Result<GLuint, String> {
    let program: GLuint = gl::CreateProgram();

    gl::AttachShader(program, vertex_shader);
    gl::AttachShader(program, fragment_shader);

    // Link program
    gl::LinkProgram(program);
    // Check for errors
    let mut program_success: GLint = gl::TRUE as i32;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut program_success);
    if program_success != gl::TRUE as i32 {
        return Err(format!("Error linking program {}!\n", program));
    }
    return Ok(program);
}

pub unsafe fn create_shader(source: &ffi::CString, shader_type: GLuint) -> Result<GLuint, String> {

    // Create vertex shader
    let shader: GLuint = gl::CreateShader(shader_type);
    // Get vertex source
    // Set vertex source
    gl::ShaderSource(shader, 1, [source.as_ptr()].as_ptr(), ptr::null());
    // Compile vertex source
    gl::CompileShader(shader);
    // Check vertex shader for errors
    let mut shader_compiled: GLint = gl::FALSE as i32;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut shader_compiled);
    if shader_compiled != gl::TRUE as i32 {
        return Err(format!("Unable to compile shader: {:?}\n{}",
                           source,
                           print_shader_log(shader)));
    }
    return Ok(shader);
}

unsafe fn print_shader_log(shader: GLuint) -> String {
    // Make sure name is shader
    if gl::IsShader(shader) > 0 {
        // Shader log length
        let mut info_log_length = 0;
        let mut max_length = info_log_length;
        // Get info string length
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut max_length);
        // Allocate string
        let info_log = ffi::CString::from_vec_unchecked(Vec::with_capacity(max_length as usize))
            .into_raw();
        // Get info log
        gl::GetShaderInfoLog(shader, max_length, &mut info_log_length, info_log);
        if info_log_length > 0 {
            // Print Log
            let info_string = ffi::CString::from_raw(info_log);
            return format!("{}\n", info_string.into_string().unwrap());
        }
    }
    format!("Name {} is not a shader\n", shader)
}


pub struct VertexAttribArray {
    loc: GLuint,
}

impl VertexAttribArray {
    pub unsafe fn new(name: &str, shader: GLuint) -> VertexAttribArray {
        let name = ffi::CString::new(name.as_bytes()).unwrap();
        let loc = gl::GetAttribLocation(shader, name.as_ptr()) as u32;// TODO CHECK

        return VertexAttribArray { loc: loc };
    }

    pub unsafe fn enable(&self) {
        gl::EnableVertexAttribArray(self.loc);
    }

    pub unsafe fn vertex_attrib_pointer(&self,
                                        size: GLint,
                                        gl_type: GLenum,
                                        stride: GLsizei,
                                        offset: *const c_void) {
        gl::VertexAttribPointer(self.loc, size, gl_type, gl::FALSE, stride, offset);
    }

    pub unsafe fn disable(&self) {
        gl::DisableVertexAttribArray(self.loc);
    }
}
