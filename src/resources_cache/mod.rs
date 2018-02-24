use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use alto;

use sdl2::image::LoadTexture;

use font::BitmapFont;

use std::rc::Rc;

mod cache;
mod load;
mod keys;

pub use self::cache::*;
pub use self::load::*;
pub use self::keys::*;

pub struct Resources {
    texture_cache: HashCache<PathKey, Texture>,
    bitmap_font_cache: HashCache<BitmapFontKey, BitmapFont>,
    texture_creator: TextureCreator<WindowContext>,
    alto_context: alto::Context, // TODO Check that it is ok to clone this (are they the same context?)
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
        }
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
    fn get(&self, key: &PathKey) -> Option<Rc<Texture>> {
        self.texture_cache.get(key)
    }

    fn insert(&mut self, key: PathKey, value: Texture) -> Option<Rc<Texture>> {
        self.texture_cache.insert(key, value)
    }

    fn remove(&mut self, key: &PathKey) -> Option<Rc<Texture>> {
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
    fn get(&self, key: &BitmapFontKey) -> Option<Rc<BitmapFont>> {
        self.bitmap_font_cache.get(key)
    }

    fn insert(&mut self, key: BitmapFontKey, value: BitmapFont) -> Option<Rc<BitmapFont>> {
        self.bitmap_font_cache.insert(key, value)
    }

    fn remove(&mut self, key: &BitmapFontKey) -> Option<Rc<BitmapFont>> {
        self.bitmap_font_cache.remove(key)
    }

    fn clear(&mut self) {
        self.bitmap_font_cache.clear();
    }
}
