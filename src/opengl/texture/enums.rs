#![allow(unused)]

use gl;
use gl::types::*;

/// Specifies the target texture.
#[derive(Clone, Copy, Debug)]
pub enum TextureTarget {
    Texture2D,
    ProxyTexture2D,
    Texture1dArray,
    ProxyTexture1dArray,
    TextureRectangle,
    ProxyTextureRectangle,
    TextureCubeMapPositiveX,
    TextureCubeMapNegativeX,
    TextureCubeMapPositiveY,
    TextureCubeMapNegativeY,
    TextureCubeMapPositiveZ,
    TextureCubeMapNegativeZ,
    ProxyTextureCubeMap,
}

impl Into<GLenum> for TextureTarget {
    fn into(self) -> GLenum {
        use self::TextureTarget::*;
        match self {
            Texture2D => gl::TEXTURE_2D,
            ProxyTexture2D => gl::PROXY_TEXTURE_2D,
            Texture1dArray => gl::TEXTURE_1D_ARRAY,
            ProxyTexture1dArray => gl::PROXY_TEXTURE_1D_ARRAY,
            TextureRectangle => gl::TEXTURE_RECTANGLE,
            ProxyTextureRectangle => gl::PROXY_TEXTURE_RECTANGLE,
            TextureCubeMapPositiveX => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
            TextureCubeMapNegativeX => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
            TextureCubeMapPositiveY => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
            TextureCubeMapNegativeY => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            TextureCubeMapPositiveZ => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
            TextureCubeMapNegativeZ => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
            ProxyTextureCubeMap => gl::PROXY_TEXTURE_CUBE_MAP,
        }
    }
}

/// Specifies the format of the pixel data.
#[derive(Clone, Copy, Debug)]
pub enum TextureFormat {
    GlRed,
    GlRg,
    GlRgb,
    GlBgr,
    GlRgba,
    GlBgra,
    GlRedInteger,
    GlRgInteger,
    GlRgbInteger,
    GlBgrInteger,
    GlRgbaInteger,
    GlBgraInteger,
    GlStencilIndex,
    GlDepthComponent,
    GlDepthStencil,
}

impl Into<GLenum> for TextureFormat {
    fn into(self) -> GLenum {
        use self::TextureFormat::*;
        match self {
            GlRed => gl::RED,
            GlRg => gl::RG,
            GlRgb => gl::RGB,
            GlBgr => gl::BGR,
            GlRgba => gl::RGBA,
            GlBgra => gl::BGRA,
            GlRedInteger => gl::RED_INTEGER,
            GlRgInteger => gl::RG_INTEGER,
            GlRgbInteger => gl::RGB_INTEGER,
            GlBgrInteger => gl::BGR_INTEGER,
            GlRgbaInteger => gl::RGBA_INTEGER,
            GlBgraInteger => gl::BGRA_INTEGER,
            GlStencilIndex => gl::STENCIL_INDEX,
            GlDepthComponent => gl::DEPTH_COMPONENT,
            GlDepthStencil => gl::DEPTH_STENCIL,
        }
    }
}

/// Specifies the number of color components in the texture.
#[derive(Clone, Copy, Debug)]
pub enum TextureInternalFormat {
    Depth,
    DepthStencil,
    Red,
    RG,
    RGB,
    RGBA, // TODO add other internal formats
}

impl Into<GLint> for TextureInternalFormat {
    fn into(self) -> GLint {
        use self::TextureInternalFormat::*;
        let value = match self {
            Depth => gl::DEPTH_COMPONENT,
            DepthStencil => gl::DEPTH_STENCIL,
            Red => gl::RED,
            RG => gl::RG,
            RGB => gl::RGB,
            RGBA => gl::RGBA,
        };
        value as GLint
    }
}
