
use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::Rect;

pub struct BitmapFont{
    texture: Texture,
    char_width: u32,
    char_height: u32,
}


impl BitmapFont{

    // TODO maybe create from Path instead of from Texture
    // TODO also could be useful to have a function on Engine that returns the bitmap font
    pub fn new(texture: Texture, char_size: (u32, u32)) -> Self {
        BitmapFont{
            texture,
            char_width: char_size.0,
            char_height: char_size.1,
        }
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8){
        self.texture.set_color_mod(r, g, b);
    }

    // TODO improve this
    // TODO at least during debug check that the char are in the correct range (can be handled)
    pub fn render_text(&mut self, text: &str, point: (i32, i32), renderer: &mut WindowCanvas) {
        let width = self.texture.query().width;
        let width = width - width % self.char_width;

        for (c, character) in text.char_indices(){
            let char_index = character as usize - 32;

            let x = ((char_index as u32 * self.char_width) % width) as i32;
            let y = ((char_index as u32 * self.char_width / width) * self.char_height) as i32;


            let rect = Rect::new(x,y, self.char_width, self.char_height);

            let dest_x = (c as u32 * self.char_width % width) as i32;
            let dest_y = ((c as u32 / width) * self.char_height) as i32;

            let mut dest_rect = Rect::new(dest_x, dest_y, self.char_width, self.char_height);
            dest_rect.x += point.0;
            dest_rect.y += point.1;

            renderer.copy(&self.texture, rect, dest_rect).unwrap();
        }
    }

    // TODO add other functions (to print text centered or right aligned)

    // TODO consider handling '\n' (just go to next line)

}
