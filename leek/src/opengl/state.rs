use gl;

use gl::types::GLboolean;

// TODO use right GL types instead of i32 and u32
// TODO add missing states (ex: texture other than TEXTURE_2D)
#[derive(Debug)]
pub struct GLState {
    last_active_texture: i32,
    last_program: i32,
    last_texture: i32,
    last_sampler: i32,
    last_array_buffer: i32,
    last_element_array_buffer: i32,
    last_vertex_array: i32,
    last_polygon_mode: [i32; 2],
    last_viewport: [i32; 4],
    last_scissor_box: [i32; 4],
    last_blend_src_rgb: i32,
    last_blend_dst_rgb: i32,
    last_blend_src_alpha: i32,
    last_blend_dst_alpha: i32,
    last_blend_equation_rgb: i32,
    last_blend_equation_alpha: i32,
    last_enable_blend: GLboolean,
    last_enable_cull_face: GLboolean,
    last_enable_depth_test: GLboolean,
    last_enable_scissor_test: GLboolean,
}

impl GLState {
    pub unsafe fn snapshot() -> GLState {
        let mut last_active_texture = 0;
        gl::GetIntegerv(gl::ACTIVE_TEXTURE, &mut last_active_texture);
        gl::ActiveTexture(gl::TEXTURE0);
        let mut last_program = 0;
        gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut last_program);
        let mut last_texture = 0;
        gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut last_texture);
        let mut last_sampler = 0;
        gl::GetIntegerv(gl::SAMPLER_BINDING, &mut last_sampler);
        let mut last_array_buffer = 0;
        gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut last_array_buffer);
        let mut last_element_array_buffer = 0;
        gl::GetIntegerv(
            gl::ELEMENT_ARRAY_BUFFER_BINDING,
            &mut last_element_array_buffer,
        );
        let mut last_vertex_array = 0;
        gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut last_vertex_array);

        let mut last_polygon_mode = [0; 2];
        gl::GetIntegerv(gl::POLYGON_MODE, last_polygon_mode.as_mut_ptr() as *mut i32);
        let mut last_viewport = [0; 4];
        gl::GetIntegerv(gl::VIEWPORT, last_viewport.as_mut_ptr());
        let mut last_scissor_box = [0; 4];
        gl::GetIntegerv(gl::SCISSOR_BOX, last_scissor_box.as_mut_ptr());

        let mut last_blend_src_rgb = 0;
        gl::GetIntegerv(gl::BLEND_SRC_RGB, &mut last_blend_src_rgb);
        let mut last_blend_dst_rgb = 0;
        gl::GetIntegerv(gl::BLEND_DST_RGB, &mut last_blend_dst_rgb);
        let mut last_blend_src_alpha = 0;
        gl::GetIntegerv(gl::BLEND_SRC_ALPHA, &mut last_blend_src_alpha);
        let mut last_blend_dst_alpha = 0;
        gl::GetIntegerv(gl::BLEND_DST_ALPHA, &mut last_blend_dst_alpha);
        let mut last_blend_equation_rgb = 0;
        gl::GetIntegerv(gl::BLEND_EQUATION_RGB, &mut last_blend_equation_rgb);
        let mut last_blend_equation_alpha = 0;
        gl::GetIntegerv(gl::BLEND_EQUATION_ALPHA, &mut last_blend_equation_alpha);
        let last_enable_blend = gl::IsEnabled(gl::BLEND);
        let last_enable_cull_face = gl::IsEnabled(gl::CULL_FACE);
        let last_enable_depth_test = gl::IsEnabled(gl::DEPTH_TEST);
        let last_enable_scissor_test = gl::IsEnabled(gl::SCISSOR_TEST);

        GLState {
            last_active_texture,
            last_program,
            last_texture,
            last_sampler,
            last_array_buffer,
            last_element_array_buffer,
            last_vertex_array,
            last_polygon_mode,
            last_viewport,
            last_scissor_box,
            last_blend_src_rgb,
            last_blend_dst_rgb,
            last_blend_src_alpha,
            last_blend_dst_alpha,
            last_blend_equation_rgb,
            last_blend_equation_alpha,
            last_enable_blend,
            last_enable_cull_face,
            last_enable_depth_test,
            last_enable_scissor_test,
        }
    }

    pub unsafe fn restore(self) {
        gl::UseProgram(self.last_program as u32);
        gl::BindTexture(gl::TEXTURE_2D, self.last_texture as u32);
        gl::BindSampler(0, self.last_sampler as u32);
        gl::ActiveTexture(self.last_active_texture as u32);
        gl::BindVertexArray(self.last_vertex_array as u32);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.last_array_buffer as u32);
        gl::BindBuffer(
            gl::ELEMENT_ARRAY_BUFFER,
            self.last_element_array_buffer as u32,
        );
        gl::BlendEquationSeparate(
            self.last_blend_equation_rgb as u32,
            self.last_blend_equation_alpha as u32,
        );
        gl::BlendFuncSeparate(
            self.last_blend_src_rgb as u32,
            self.last_blend_dst_rgb as u32,
            self.last_blend_src_alpha as u32,
            self.last_blend_dst_alpha as u32,
        );
        if self.last_enable_blend == 1 {
            gl::Enable(gl::BLEND);
        } else {
            gl::Disable(gl::BLEND);
        }
        if self.last_enable_cull_face == 1 {
            gl::Enable(gl::CULL_FACE);
        } else {
            gl::Disable(gl::CULL_FACE);
        }
        if self.last_enable_depth_test == 1 {
            gl::Enable(gl::DEPTH_TEST);
        } else {
            gl::Disable(gl::DEPTH_TEST);
        }
        if self.last_enable_scissor_test == 1 {
            gl::Enable(gl::SCISSOR_TEST);
        } else {
            gl::Disable(gl::SCISSOR_TEST);
        }
        gl::PolygonMode(gl::FRONT_AND_BACK, self.last_polygon_mode[0] as u32);
        gl::Viewport(
            self.last_viewport[0],
            self.last_viewport[1],
            self.last_viewport[2],
            self.last_viewport[3],
        );
        gl::Scissor(
            self.last_scissor_box[0],
            self.last_scissor_box[1],
            self.last_scissor_box[2],
            self.last_scissor_box[3],
        );
    }
}
