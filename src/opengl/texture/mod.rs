use gl;
use gl::types::*;

use std::os::raw::c_void;

mod enums;
pub use self::enums::*;

use opengl::Type;

#[derive(Debug)]
pub struct Texture {
    loc: GLuint,
}

impl Texture {
    pub fn new(
        width: u32,
        height: u32,
        data: &[u8],
        texture_target: TextureTarget,
        texture_internal_format: TextureInternalFormat,
        target_format: TextureFormat,
        data_type: Type,
    ) -> Result<Texture, String> {
        unsafe {
            let mut texture_loc = 0;
            gl::GenTextures(1, &mut texture_loc);

            let texture_target = texture_target.into();

            gl::BindTexture(texture_target, texture_loc);
            gl::TexParameteri(texture_target, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(texture_target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);
            gl::TexImage2D(
                texture_target,
                0, // Specifies the level-of-detail number. Level 0 is the base image level. Level n is the nth mipmap reduction image. If target is GL_TEXTURE_RECTANGLE or GL_PROXY_TEXTURE_RECTANGLE, level must be 0.
                texture_internal_format.into(),
                width as i32,
                height as i32,
                0, // must be 0 (according to opengl doc)
                target_format.into(),
                data_type.into(),
                data.as_ptr() as *const c_void,
            );

            Ok(Texture { loc: texture_loc })
        }
    }

    pub fn get_id(&self) -> usize {
        self.loc as usize
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.loc);
        }
    }
}
