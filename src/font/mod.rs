use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::Rect;

pub struct BitmapFont {
    texture: Texture,
    char_width: u32,
    char_height: u32,
}

impl BitmapFont {
    // TODO maybe create from Path instead of from Texture
    // TODO also could be useful to have a function on Engine that returns the bitmap font
    pub fn new(texture: Texture, char_size: (u32, u32)) -> Self {
        BitmapFont {
            texture,
            char_width: char_size.0,
            char_height: char_size.1,
        }
    }

    pub fn vram_size(&self) -> usize {
        let tex_query = self.texture.query();

        let pixels = tex_query.width * tex_query.height;
        tex_query.format.byte_size_of_pixels(pixels as usize)
    }

    pub fn set_color(&self, red: u8, green: u8, blue: u8) {
        unsafe {
            use sdl2::sys;
            use sdl2::get_error;

            let raw = self.texture.raw();
            let ret = sys::SDL_SetTextureColorMod(raw, red, green, blue);
            if ret != 0 {
                panic!("Error setting color mod: {}", get_error())
            }
        }
    }

    // TODO improve this
    // TODO at least during debug check that the char are in the correct range (can be handled)
    pub fn render_text(&self, text: &str, point: (i32, i32), renderer: &mut WindowCanvas) {
        let width = self.texture.query().width;
        let width = width - width % self.char_width;

        let mut line = 0;

        // This is needed to rebalance the index when going on a new line \n
        let mut x_counter_offset = 0;

        for (char_count, character) in text.char_indices() {
            if character == '\n' {
                line += 1;
                x_counter_offset = char_count + 1;
                continue;
            }

            let char_index = character as usize - 32;

            let src_x = ((char_index as u32 * self.char_width) % width) as i32;
            let src_y = ((char_index as u32 * self.char_width / width) * self.char_height) as i32;
            let src_rect = Rect::new(src_x, src_y, self.char_width, self.char_height);

            let dest_x = ((char_count - x_counter_offset) as u32 * self.char_width) as i32;
            let dest_y = (line * self.char_height) as i32;

            let dest_rect = Rect::new(
                dest_x + point.0,
                dest_y + point.1,
                self.char_width,
                self.char_height,
            );

            renderer.copy(&self.texture, src_rect, dest_rect).unwrap();
        }
    }

    // TODO add other functions (to print text centered or right aligned)

    pub fn render_text_centered(&self, text: &str, point: (i32, i32), renderer: &mut WindowCanvas) {
        let mut n_lines = 0;
        let mut largest_line = 0;
        for line in text.lines() {
            largest_line = largest_line.max(line.chars().count());
            n_lines += 1;
        }

        let width = (largest_line as u32 * self.char_width) as i32;
        let heigth = (n_lines as u32 * self.char_height) as i32;

        let x = point.0 - width / 2;
        let y = point.1 - heigth / 2;

        self.render_text(text, (x, y), renderer)
    }
}
