use sdl2::render::{Texture, TextureAccess, TextureCreator, TextureValueError};
use sdl2::surface::SurfaceRef;
use sdl2::video::WindowContext;
use sdl2::pixels::PixelFormatEnum;
use sdl2::image::LoadTexture;

use alto;
use alto_utils::load_buffer_from_ogg_file;

use font::BitmapFont;

use std::sync::Arc;

use imgui::Ui;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::time::Duration;

mod cache;
mod load;
mod keys;

pub use self::cache::*;
pub use self::load::*;
pub use self::keys::*;

pub struct Resources {
    pub inspect_window: bool,
    texture_cache: HashCache<PathKey, Texture>,
    bitmap_font_cache: HashCache<BitmapFontKey, BitmapFont>,
    audio_buffer_cache: HashCache<PathKey, alto::Buffer>,
    texture_creator: TextureCreator<WindowContext>,
    alto_context: alto::Context,
    watcher: RecommendedWatcher,
    receiver: Receiver<DebouncedEvent>,
}

impl Resources {
    #[cfg(debug_assertions)]
    fn init_assets_folder() {
        use std::path::Path;
        use std::fs;

        let asset_folder: &Path = Path::new("./assets");
        if !asset_folder.exists() {
            fs::create_dir(asset_folder);
        }
    }

    pub fn new(
        texture_creator: TextureCreator<WindowContext>,
        alto_context: alto::Context,
    ) -> Self {
        let (tx, receiver) = channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(500)).unwrap();

        #[cfg(debug_assertions)]
        Resources::init_assets_folder();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch("assets", RecursiveMode::Recursive).unwrap();

        Resources {
            inspect_window: false,
            texture_creator,
            alto_context,
            texture_cache: Default::default(),
            bitmap_font_cache: Default::default(),
            audio_buffer_cache: Default::default(),
            watcher,
            receiver,
        }
    }

    pub fn sync_resources(&mut self) -> Option<PathKey> {
        match self.receiver.try_recv() {
            Ok(event) => match event {
                DebouncedEvent::Write(path) => {
                    let key: Option<PathKey> = self.texture_cache
                        .into_iter()
                        .filter(|&(k, _)| path.ends_with(k.0))
                        .map(|(ref k, _)| (*k).clone())
                        .nth(0);

                    if let Some(key) = key {
                        if let Ok(new_value) = self.load_resource(&key) {
                            println!("Reloaded {}", key.0);
                            self.texture_cache.insert(key.clone(), new_value);
                            return Some(key);
                        } else {
                            println!("Error during reloading {}", key.0);
                        }
                    }

                    // Same for audio

                    let key: Option<PathKey> = self.audio_buffer_cache
                        .into_iter()
                        .filter(|&(k, _)| path.ends_with(k.0))
                        .map(|(ref k, _)| (*k).clone())
                        .nth(0);

                    if let Some(key) = key {
                        if let Ok(new_value) = self.load_resource(&key) {
                            println!("Reloaded {}", key.0);
                            self.audio_buffer_cache.insert(key.clone(), new_value);
                            return Some(key);
                        } else {
                            println!("Error during reloading {}", key.0);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        None
    }

    pub fn inspect(&mut self, ui: &Ui) {
        use std::mem::size_of;
        use math::format_bytes;

        let mut opened = self.inspect_window;

        ui.window(im_str!("Resources"))
            .scrollable(true)
            .resizable(true)
            .opened(&mut opened)
            .build(|| {
                ui.text(im_str!("Texture cache"));
                ui.separator();

                let ram_usage = size_of::<Texture>() * self.texture_cache.size();
                let vram_estimate: usize =
                    (self.texture_cache.into_iter()).fold(0, |acc, (_, t)| {
                        let tex_query = t.query();

                        let pixels = tex_query.width * tex_query.height;
                        let size = tex_query.format.byte_size_of_pixels(pixels as usize);

                        acc + size
                    });

                ui.text(im_str!(
                    "Using {} of RAM and {} of VRAM",
                    format_bytes(ram_usage as f64),
                    format_bytes(vram_estimate as f64)
                ));
                if ui.is_item_hovered() {
                    ui.tooltip_text(im_str!(
                        "{} bytes of RAM and {} bytes of VRAM",
                        ram_usage,
                        vram_estimate
                    ))
                }

                self.texture_cache.inspect(
                    ui,
                    "texture_cache",
                    |key| key.0.trim_left_matches("assets/textures/"),
                    |value| {
                        let tex_query = value.query();

                        let pixels = tex_query.width * tex_query.height;
                        tex_query.format.byte_size_of_pixels(pixels as usize)
                    },
                );
                // -----
                ui.new_line();
                ui.text(im_str!("Audio cache"));
                ui.separator();

                let ram_usage = size_of::<Texture>() * self.audio_buffer_cache.size();
                let ram_estimate: usize = (self.audio_buffer_cache.into_iter())
                    .fold(0, |acc, (_, buffer)| acc + buffer.size() as usize);

                ui.text(im_str!(
                    "Using {} of RAM ",
                    format_bytes(ram_usage as f64 + ram_estimate as f64)
                ));
                if ui.is_item_hovered() {
                    ui.tooltip_text(im_str!("{} bytes of RAM", ram_usage + ram_estimate))
                }

                self.audio_buffer_cache.inspect(
                    ui,
                    "audio_buffer_cache",
                    |key| key.0.trim_left_matches("assets/sounds/"),
                    |value| value.size() as usize,
                );

                // -----
                ui.new_line();
                ui.text(im_str!("Font cache cache"));
                ui.separator();

                let ram_usage = size_of::<Texture>() * self.bitmap_font_cache.size();
                let vram_estimate: usize = (self.bitmap_font_cache.into_iter())
                    .fold(0, |acc, (_, bfont)| acc + bfont.vram_size());

                ui.text(im_str!(
                    "Using {} of RAM and an estimate {} of VRAM ",
                    format_bytes(ram_usage as f64),
                    format_bytes(vram_estimate as f64)
                ));
                if ui.is_item_hovered() {
                    ui.tooltip_text(im_str!(
                        "{} bytes of RAM and {} bytes of VRAM",
                        ram_usage,
                        vram_estimate
                    ))
                }

                self.bitmap_font_cache.inspect(
                    ui,
                    "bitmap_font_cache",
                    |key| key.0.trim_left_matches("assets/fonts/"),
                    |value| value.vram_size(),
                );
            });

        self.inspect_window = opened;
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

    fn size(&self) -> usize {
        self.texture_cache.size()
    }
    fn drop_unused(&mut self) {
        self.texture_cache.drop_unused()
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

    fn size(&self) -> usize {
        self.bitmap_font_cache.size()
    }
    fn drop_unused(&mut self) {
        self.texture_cache.drop_unused()
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

    fn size(&self) -> usize {
        self.audio_buffer_cache.size()
    }
    fn drop_unused(&mut self) {
        self.texture_cache.drop_unused()
    }
}
