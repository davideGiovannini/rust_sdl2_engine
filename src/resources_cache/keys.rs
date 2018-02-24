use sdl2::render::Texture;

use font::BitmapFont;

pub trait CacheKey<Target> {
    type Target;
}

/// Key made of a path
#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct PathKey(pub String);

impl CacheKey<Texture> for PathKey {
    type Target = Texture;
}

impl<'a> From<&'a str> for PathKey {
    fn from(s: &str) -> Self {
        PathKey(s.to_string())
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
