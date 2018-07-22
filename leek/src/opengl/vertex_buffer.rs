#![allow(unused)]
// TODO remove this annotation and actually use this stuff

use gl;
use gl::types::*;

use std::os::raw::c_void;
use std::ptr;

unsafe fn initialize_empty_buffer(gl_type: u32) -> GLuint {
    let mut loc: GLuint = 0;
    gl::GenBuffers(1, &mut loc);

    gl::BindBuffer(gl_type, loc);
    gl::BufferData(gl_type, 0 as GLsizeiptr, ptr::null(), gl::DYNAMIC_DRAW);
    // TODO check for failure and return error
    loc
}

#[derive(Debug)]
pub struct VertexBuffer {
    loc: GLuint,
}

impl VertexBuffer {
    pub fn new_empty_dynamic() -> Result<VertexBuffer, String> {
        unsafe {
            Ok(VertexBuffer {
                loc: initialize_empty_buffer(gl::ARRAY_BUFFER),
            })
        }
    }

    pub fn upload_new_data(&mut self, vtx_buffer: &[u8]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.loc);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                vtx_buffer.len() as GLsizeiptr,
                vtx_buffer.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW,
            );
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.loc);
        }
    }
}

#[derive(Debug)]
pub struct ElementBuffer {
    loc: GLuint,
}

impl ElementBuffer {
    pub fn new_empty_dynamic() -> Result<ElementBuffer, String> {
        unsafe {
            Ok(ElementBuffer {
                loc: initialize_empty_buffer(gl::ELEMENT_ARRAY_BUFFER),
            })
        }
    }

    pub fn upload_new_data(&mut self, idx_buffer: &[u8]) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.loc);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                idx_buffer.len() as GLsizeiptr,
                idx_buffer.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW,
            );
        }
    }
}

impl Drop for ElementBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.loc);
        }
    }
}
