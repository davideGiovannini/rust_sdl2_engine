use gl;
use gl::types::*;

use std::{ffi, ptr};

#[derive(Clone, Copy, Debug)]
pub struct UniformLocation(GLuint);

impl Into<GLint> for UniformLocation {
    fn into(self) -> GLint {
        self.0 as GLint
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AttribLocation(GLuint);

impl Into<GLuint> for AttribLocation {
    fn into(self) -> GLuint {
        self.0
    }
}

impl AttribLocation {
    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray((*self).into());
        }
    }
}

#[derive(Debug)]
pub struct Shader {
    raw_program: GLuint,
    raw_vertex: GLuint,
    raw_fragment: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DetachShader(self.raw_program, self.raw_vertex);
            gl::DetachShader(self.raw_program, self.raw_fragment);
            gl::DeleteProgram(self.raw_program);
            gl::DeleteShader(self.raw_vertex);
            gl::DeleteShader(self.raw_fragment);
        }
    }
}

impl Shader {
    pub fn new(vertex_source: &str, fragment_source: &str) -> Result<Shader, String> {
        unsafe {
            let vertex_source =
                ffi::CString::new(vertex_source.as_bytes()).map_err(|_| "CString Null error")?;
            let fragment_source =
                ffi::CString::new(fragment_source.as_bytes()).map_err(|_| "CString Null error")?;

            let vertex_shader = create_shader(&vertex_source, gl::VERTEX_SHADER)?;
            let fragment_shader = create_shader(&fragment_source, gl::FRAGMENT_SHADER)?;

            let program = compile_program(vertex_shader, fragment_shader)?;
            Ok(Shader {
                raw_program: program,
                raw_vertex: vertex_shader,
                raw_fragment: fragment_shader,
            })
        }
    }
    pub fn raw(&self) -> GLuint {
        self.raw_program
    }

    pub unsafe fn get_attrib_location(&self, name: &str) -> AttribLocation {
        let cname = ffi::CString::new(name.as_bytes()).unwrap();
        let v = gl::GetAttribLocation(self.raw_program, cname.as_ptr() as *const i8) as u32;
        AttribLocation(v)
    }
    pub unsafe fn get_uniform_location(&self, name: &str) -> UniformLocation {
        let cname = ffi::CString::new(name.as_bytes()).unwrap();
        let v = gl::GetUniformLocation(self.raw_program, cname.as_ptr() as *const i8) as u32;
        UniformLocation(v)
    }

    pub unsafe fn activate(&self) {
        gl::UseProgram(self.raw_program);
    }
}

unsafe fn compile_program(
    vertex_shader: GLuint,
    fragment_shader: GLuint,
) -> Result<GLuint, String> {
    let program: GLuint = gl::CreateProgram();

    gl::AttachShader(program, vertex_shader);
    gl::AttachShader(program, fragment_shader);

    // Link program
    gl::LinkProgram(program);
    // Check for errors
    let mut program_success: GLint = i32::from(gl::TRUE);
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut program_success);
    if program_success != i32::from(gl::TRUE) {
        return Err(format!("Error linking program {}!\n", program));
    }
    Ok(program)
}

unsafe fn create_shader(source: &ffi::CString, shader_type: GLuint) -> Result<GLuint, String> {
    // Create vertex shader
    let shader: GLuint = gl::CreateShader(shader_type);
    // Get vertex source
    // Set vertex source
    gl::ShaderSource(shader, 1, [source.as_ptr()].as_ptr(), ptr::null());
    // Compile vertex source
    gl::CompileShader(shader);
    // Check vertex shader for errors
    let mut shader_compiled: GLint = i32::from(gl::FALSE);
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut shader_compiled);
    if shader_compiled != i32::from(gl::TRUE) {
        return Err(format!(
            "Unable to compile shader: {:?}\n{}",
            source,
            print_shader_log(shader)
        ));
    }
    Ok(shader)
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
        let info_log =
            ffi::CString::from_vec_unchecked(Vec::with_capacity(max_length as usize)).into_raw();
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
