
use gl;
use gl::types::*;

use std::ffi;
use std::os::raw::c_void;


pub struct VertexAttribArray {
    loc: GLuint,
}

impl VertexAttribArray {
    pub unsafe fn new(name: &str, shader: GLuint) -> VertexAttribArray {
        let name = ffi::CString::new(name.as_bytes()).unwrap();
        let loc = gl::GetAttribLocation(shader, name.as_ptr()) as u32; // TODO CHECK

        return VertexAttribArray { loc };
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
