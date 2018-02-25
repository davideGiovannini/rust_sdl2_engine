use sdl2::render::{Texture, TextureAccess, TextureCreator, TextureValueError};
use sdl2::surface::SurfaceRef;
use sdl2::video::WindowContext;
use sdl2::pixels::PixelFormatEnum;
use sdl2::image::LoadTexture;

use alto;
use alto_utils::load_buffer_from_ogg_file;

use font::BitmapFont;

use std::sync::Arc;

mod cache;
mod load;
mod keys;

pub use self::cache::*;
pub use self::load::*;
pub use self::keys::*;

pub struct Resources {
    texture_cache: HashCache<PathKey, Texture>,
    bitmap_font_cache: HashCache<BitmapFontKey, BitmapFont>,
    audio_buffer_cache: HashCache<PathKey, alto::Buffer>,
    texture_creator: TextureCreator<WindowContext>,
    alto_context: alto::Context,
}

impl Resources {
    pub fn new(
        texture_creator: TextureCreator<WindowContext>,
        alto_context: alto::Context,
    ) -> Self {
        Resources {
            texture_creator,
            alto_context,
            texture_cache: Default::default(),
            bitmap_font_cache: Default::default(),
            audio_buffer_cache: Default::default(),
        }
    }

    pub fn default_pixel_format(&self) -> PixelFormatEnum {
        self.texture_creator.default_pixel_format()
    }
    pub fn create_texture<F>(
        &self,
        format: F,
        access: TextureAccess,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureValueError>
    where
        F: Into<Option<PixelFormatEnum>>,
    {
        self.texture_creator
            .create_texture(format, access, width, height)
    }

    pub fn create_texture_static<F>(
        &self,
        format: F,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureValueError>
    where
        F: Into<Option<PixelFormatEnum>>,
    {
        self.texture_creator
            .create_texture_static(format, width, height)
    }

    pub fn create_texture_streaming<F>(
        &self,
        format: F,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureValueError>
    where
        F: Into<Option<PixelFormatEnum>>,
    {
        self.texture_creator
            .create_texture_streaming(format, width, height)
    }

    pub fn create_texture_target<F>(
        &self,
        format: F,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureValueError>
    where
        F: Into<Option<PixelFormatEnum>>,
    {
        self.texture_creator
            .create_texture_target(format, width, height)
    }

    pub fn create_texture_from_surface<S: AsRef<SurfaceRef>>(
        &self,
        surface: S,
    ) -> Result<Texture, TextureValueError> {
        self.texture_creator.create_texture_from_surface(surface)
    }
}

impl LoadCache<PathKey, Texture> for Resources {}

impl Loader<PathKey, Texture> for Resources {
    type Error = String;

    fn load_resource(&self, key: &PathKey) -> Result<Texture, Self::Error> {
        self.texture_creator.load_texture(&key.0)
    }
}

impl Cache<PathKey, Texture> for Resources {
    fn get(&self, key: &PathKey) -> Option<Arc<Texture>> {
        self.texture_cache.get(key)
    }

    fn insert(&mut self, key: PathKey, value: Texture) -> Option<Arc<Texture>> {
        self.texture_cache.insert(key, value)
    }

    fn remove(&mut self, key: &PathKey) -> Option<Arc<Texture>> {
        self.texture_cache.remove(key)
    }

    fn clear(&mut self) {
        self.texture_cache.clear();
    }
}

impl LoadCache<BitmapFontKey, BitmapFont> for Resources {}

impl Loader<BitmapFontKey, BitmapFont> for Resources {
    type Error = String;

    fn load_resource(&self, key: &BitmapFontKey) -> Result<BitmapFont, Self::Error> {
        let font_texture = self.texture_creator.load_texture(&key.0)?;

        Ok(BitmapFont::new(font_texture, key.1))
    }
}

impl Cache<BitmapFontKey, BitmapFont> for Resources {
    fn get(&self, key: &BitmapFontKey) -> Option<Arc<BitmapFont>> {
        self.bitmap_font_cache.get(key)
    }

    fn insert(&mut self, key: BitmapFontKey, value: BitmapFont) -> Option<Arc<BitmapFont>> {
        self.bitmap_font_cache.insert(key, value)
    }

    fn remove(&mut self, key: &BitmapFontKey) -> Option<Arc<BitmapFont>> {
        self.bitmap_font_cache.remove(key)
    }

    fn clear(&mut self) {
        self.bitmap_font_cache.clear();
    }
}

impl LoadCache<PathKey, alto::Buffer> for Resources {}

impl Loader<PathKey, alto::Buffer> for Resources {
    type Error = String;

    fn load_resource(&self, key: &PathKey) -> Result<alto::Buffer, Self::Error> {
        load_buffer_from_ogg_file(&key.0, &self.alto_context).map_err(|e| format!("{}", e))
    }
}

impl Cache<PathKey, alto::Buffer> for Resources {
    fn get(&self, key: &PathKey) -> Option<Arc<alto::Buffer>> {
        self.audio_buffer_cache.get(key)
    }

    fn insert(&mut self, key: PathKey, value: alto::Buffer) -> Option<Arc<alto::Buffer>> {
        self.audio_buffer_cache.insert(key, value)
    }

    fn remove(&mut self, key: &PathKey) -> Option<Arc<alto::Buffer>> {
        self.audio_buffer_cache.remove(key)
    }

    fn clear(&mut self) {
        self.audio_buffer_cache.clear();
    }
}
