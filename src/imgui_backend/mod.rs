use imgui::{DrawList, ImDrawIdx, ImDrawVert, ImGui, Ui};

use opengl::*;

mod input;
pub use self::input::*;

use std::mem;
use gl;
use gl::types::GLuint;

const VERTEX_SIZE: usize = mem::size_of::<ImDrawVert>();
const INDEX_SIZE: usize = mem::size_of::<ImDrawIdx>();

pub type RendererResult<T> = Result<T, String>;

pub struct Renderer {
    device_objects: DeviceObjects,
}

impl Renderer {
    pub fn init(imgui: &mut ImGui) -> RendererResult<Renderer> {
        let device_objects = DeviceObjects::init(imgui)?;
        Ok(Renderer { device_objects })
    }

    pub fn render<'a>(&mut self, ui: Ui<'a>) -> RendererResult<()> {
        ui.render(|ui, draw_list| self.render_draw_list(ui, &draw_list))
    }

    fn render_draw_list<'a>(
        &mut self,
        ui: &'a Ui<'a>,
        draw_list: &DrawList<'a>,
    ) -> RendererResult<()> {
        let (width, height) = ui.imgui().display_size();
        let (scale_width, scale_height) = ui.imgui().display_framebuffer_scale();

        let (fb_width, fb_height) = (width * scale_width, height * scale_height);

        if fb_width == 0.0 || fb_height == 0.0 {
            return Ok(());
        }

        let matrix = [
            [2.0 / width as f32, 0.0, 0.0, 0.0],
            [0.0, -2.0 / (height as f32), 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [-1.0, 1.0, 0.0, 1.0],
        ];
        let font_texture_id = self.device_objects.texture.get_id();

        unsafe {
            let prev_gl_state = GLState::snapshot();

            gl::Viewport(0, 0, fb_width as i32, fb_height as i32);

            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::SCISSOR_TEST);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

            gl::Enable(gl::DEBUG_OUTPUT);

            gl::UseProgram(self.device_objects.program.raw());

            gl::Enable(gl::TEXTURE_2D);
            gl::ActiveTexture(gl::TEXTURE0);

            gl::UniformMatrix4fv(
                self.device_objects.u_projmat.into(),
                1,
                0,
                matrix.as_ptr() as *const f32,
            );
            gl::Uniform1i(self.device_objects.u_texture.into(), 0);

            gl::BindVertexArray(self.device_objects.vao);

            gl::BindSampler(0, 0);

            use std::os::raw::c_void;

            let mut idx_buffer_offset = 0;

            gl::BindBuffer(gl::ARRAY_BUFFER, self.device_objects.vbo_handle);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (draw_list.vtx_buffer.len() * VERTEX_SIZE) as isize,
                draw_list.vtx_buffer.as_ptr() as *const c_void,
                gl::STREAM_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.device_objects.index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (draw_list.idx_buffer.len() * INDEX_SIZE) as isize,
                draw_list.idx_buffer.as_ptr() as *const c_void,
                gl::STREAM_DRAW,
            );

            let idx_type = if INDEX_SIZE == 2 {
                gl::UNSIGNED_SHORT
            } else {
                gl::UNSIGNED_INT
            };

            for cmd in draw_list.cmd_buffer {
                assert_eq!(cmd.texture_id as usize, font_texture_id);

                gl::BindTexture(gl::TEXTURE_2D, font_texture_id as u32);

                // TODO check when scale changes
                gl::Scissor(
                    (cmd.clip_rect.x * scale_width) as i32,
                    ((height - cmd.clip_rect.w) * scale_height) as i32,
                    ((cmd.clip_rect.z - cmd.clip_rect.x) * scale_width) as i32,
                    ((cmd.clip_rect.w - cmd.clip_rect.y) * scale_height) as i32,
                );

                gl::DrawElements(
                    gl::TRIANGLES,
                    cmd.elem_count as i32,
                    idx_type,
                    (idx_buffer_offset) as *const c_void,
                );
                idx_buffer_offset += cmd.elem_count * INDEX_SIZE as u32;
            }

            // Unbind program
            gl::UseProgram(0);
            gl::Disable(gl::SCISSOR_TEST);
            gl::BindVertexArray(0);

            prev_gl_state.restore();
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct DeviceObjects {
    vbo_handle: GLuint,
    vao: GLuint,
    index_buffer: GLuint,
    program: Shader,
    texture: Texture,
    u_texture: UniformLocation,
    u_projmat: UniformLocation,
}

fn compile_default_program() -> Result<Shader, String> {
    Shader::new(
        include_str!("shaders/glsl_130.vert"),
        include_str!("shaders/glsl_130.frag"),
    )
}

impl DeviceObjects {
    pub fn init(im_gui: &mut ImGui) -> RendererResult<DeviceObjects> {
        let program = (compile_default_program())?;

        unsafe {
            let texture = create_fonts_texture(im_gui).unwrap();

            program.activate();

            let g_attrib_location_tex = program.get_uniform_location("Texture");
            let g_attrib_location_proj_mtx = program.get_uniform_location("ProjMtx");
            let g_attrib_location_position = program.get_attrib_location("Position");
            let g_attrib_location_uv = program.get_attrib_location("UV");
            let g_attrib_location_color = program.get_attrib_location("Color");

            let mut vbo = 0;
            let mut vao = 0;
            let mut ebo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            g_attrib_location_position.enable();
            g_attrib_location_uv.enable();
            g_attrib_location_color.enable();

            use std::mem::size_of;
            use std::os::raw::c_void;
            use std::ptr;

            let struct_size = size_of::<ImDrawVert>() as i32;

            gl::VertexAttribPointer(
                g_attrib_location_position.into(),
                2,
                gl::FLOAT,
                gl::FALSE,
                struct_size,
                ptr::null(),
            );
            gl::VertexAttribPointer(
                g_attrib_location_uv.into(),
                2,
                gl::FLOAT,
                gl::FALSE,
                struct_size,
                8 as *const c_void,
            );
            gl::VertexAttribPointer(
                g_attrib_location_color.into(),
                4,
                gl::UNSIGNED_BYTE,
                gl::TRUE,
                struct_size,
                16 as *const c_void,
            );

            Ok(DeviceObjects {
                vbo_handle: vbo,
                vao,
                index_buffer: ebo,
                program,
                texture,
                u_texture: g_attrib_location_tex,
                u_projmat: g_attrib_location_proj_mtx,
            })
        }
    }
}

unsafe fn create_fonts_texture(im_gui: &mut ImGui) -> Result<Texture, String> {
    use gl;

    let mut last_texture = 0;
    gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut last_texture);

    let texture = im_gui.prepare_texture(|handle| {
        Texture::new(
            handle.width,
            handle.height,
            handle.pixels,
            TextureTarget::Texture2D,
            TextureInternalFormat::RGBA,
            TextureFormat::GlRgba,
            Type::UnsignedByte,
        )
    })?;

    im_gui.set_texture_id(texture.get_id());

    // Restore state
    gl::BindTexture(gl::TEXTURE_2D, last_texture as u32);
    Ok(texture)
}

impl Drop for DeviceObjects {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo_handle);
            gl::DeleteBuffers(1, &self.index_buffer);
        }
    }
}
