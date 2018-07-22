use sdl2::render::Texture;
use alto::Buffer;

use font::BitmapFont;
use std::fmt::Debug;

pub trait CacheKey<Target>: Debug {
    type Target;
}

/// Key made of a path
#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct PathKey(pub &'static str);

impl CacheKey<Buffer> for PathKey {
    type Target = Buffer;
}

impl CacheKey<Texture> for PathKey {
    type Target = Texture;
}

impl From<&'static str> for PathKey {
    fn from(s: &'static str) -> Self {
        PathKey(s)
    }
}

/// This key contains the path to the bitmap font and the size of the char
#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct BitmapFontKey(pub String, pub (u32, u32));

impl CacheKey<BitmapFont> for BitmapFontKey {
    type Target = BitmapFont;
}

impl<'a> From<(&'a str, (u32, u32))> for BitmapFontKey {
    fn from(s: (&'a str, (u32, u32))) -> Self {
        BitmapFontKey(s.0.to_string(), s.1)
    }
}
