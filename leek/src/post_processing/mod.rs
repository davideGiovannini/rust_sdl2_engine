use opengl;

use gl;
use gl::types::*;

use self::opengl::*;

use std::ptr;

use sdl2::render::Texture;

pub struct PostProcessEffect {
    program: Shader,
    vertex_data: GLuint,
    u_framebuffer: UniformLocation,
}

impl PostProcessEffect {
    pub fn new() -> Result<PostProcessEffect, String> {
        unsafe {
            let vertex_source = include_str!("./shaders/glsl_130.vert");
            let fragment_source = include_str!("./shaders/glsl_130.frag");

            let program = Shader::new(vertex_source, fragment_source)?;

            let vertex_data: [GLfloat; 16] = [
                -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 0.0,
            ];
            let mut g_vbo: GLuint = 0;
            let mut g_vao: GLuint = 0;
            // Create VBO
            gl::GenBuffers(1, &mut g_vbo);

            gl::GenVertexArrays(1, &mut g_vao);
            gl::BindVertexArray(g_vao);

            let v_position = VertexAttribArray::new("v_position", program.raw());
            let v_uv = VertexAttribArray::new("v_uv", program.raw());

            gl::BindBuffer(gl::ARRAY_BUFFER, g_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                GL_FLOAT_SIZE as isize * 4 * 4,
                vertex_data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            v_position.enable();
            v_uv.enable();

            v_position.vertex_attrib_pointer(2, gl::FLOAT, GL_FLOAT_SIZE as i32 * 4, ptr::null());
            v_uv.vertex_attrib_pointer(
                2,
                gl::FLOAT,
                GL_FLOAT_SIZE as i32 * 4,
                (GL_FLOAT_SIZE as isize * 2) as *const _,
            );

            let u_framebuffer = program.get_uniform_location("framebuffer");

            gl::BindVertexArray(0);
            Ok(PostProcessEffect {
                program,
                vertex_data: g_vao,
                u_framebuffer,
            })
        }
    }

    pub fn render(&self, framebuffer: &mut Texture) {
        unsafe {
            gl::UseProgram(self.program.raw());

            framebuffer.gl_bind_texture();

            gl::Enable(gl::TEXTURE_2D);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::Uniform1i(self.u_framebuffer.into(), 0);

            gl::BindVertexArray(self.vertex_data);

            // render
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);

            // unbind framebuffer
            framebuffer.gl_unbind_texture();
            // Unbind program
            gl::UseProgram(0);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for PostProcessEffect {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.vertex_data].as_ptr());
            // TODO check if other gl deletes are needed
        }
    }
}
