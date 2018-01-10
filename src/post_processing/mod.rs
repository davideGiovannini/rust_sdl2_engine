use opengl;

use gl;
use gl::types::*;

use self::opengl::*;

use std::{ffi, ptr};

pub struct PostProcessEffect {
    program: GLuint,
    vertex_data: GLuint,
    v_position: VertexAttribArray,
    v_uv: VertexAttribArray,
}

impl PostProcessEffect {
    pub fn new() -> Result<PostProcessEffect, String> {
        unsafe {
            let vertex_source = ffi::CString::new(
                r#"#version 130
in vec2 v_position;
in vec2 v_uv;

out vec2 f_uv;
void main() {
    gl_Position = vec4( v_position, 0.0, 1.0 );
       f_uv = v_uv;
 }"#.as_bytes(),
            ).unwrap();
            let fragment_source = ffi::CString::new(
                "#version 130
in vec2 f_uv;
out vec4 LFragment;

uniform sampler2D framebuffer;

void main() {
     LFragment = mix(texture(framebuffer, f_uv), vec4(f_uv, 0.0, 1.0), 0.5);
}"
                    .as_bytes(),
            ).unwrap();

            let vertex_shader = create_shader(&vertex_source, gl::VERTEX_SHADER)?;
            let fragment_shader = create_shader(&fragment_source, gl::FRAGMENT_SHADER)?;

            let program = compile_program(vertex_shader, fragment_shader)?;

            let vertex_data: [GLfloat; 16] = [
                -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 0.0
            ];
            let mut g_vbo: GLuint = 0;
            // Create VBO
            gl::GenBuffers(1, &mut g_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, g_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_glfloat(4 * 4) as isize,
                vertex_data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let v_position = VertexAttribArray::new("v_position", program);
            let v_uv = VertexAttribArray::new("v_uv", program);

            Ok(PostProcessEffect {
                program,
                vertex_data: g_vbo,
                v_position,
                v_uv,
            })
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::UseProgram(self.program);

            gl::Enable(gl::TEXTURE_2D);
            gl::ActiveTexture(gl::TEXTURE0);
            let name = ffi::CString::new("framebuffer".as_bytes()).unwrap();
            let loc = gl::GetUniformLocation(self.program, name.as_ptr());

            gl::Uniform1i(loc, 0);

            // Get vertex attribute location
            // Enable vertex position
            self.v_position.enable();
            self.v_uv.enable();
            // Set vertex data
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_data);
            self.v_position
                .vertex_attrib_pointer(2, gl::FLOAT, size_of_glfloat(4), ptr::null());
            self.v_uv.vertex_attrib_pointer(
                2,
                gl::FLOAT,
                size_of_glfloat(4),
                size_of_glfloat(2) as *const _,
            );
            // render
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
            // Disable vertex position
            self.v_position.disable();
            self.v_uv.disable();
            // Unbind program
            gl::UseProgram(0);
        }
    }
}

impl Drop for PostProcessEffect {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteBuffers(1, [self.vertex_data].as_ptr());
        }
    }
}
