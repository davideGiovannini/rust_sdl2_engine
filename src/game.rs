
use sdl2::render::Renderer;
use sdl2::ttf::Sdl2TtfContext;
use EngineContext;

pub trait Game {
    fn init(renderer: &Renderer, ttf_context: Sdl2TtfContext) -> Self;

    fn set_up(&mut self);

    fn logic(&mut self, context: EngineContext);

    fn render(&mut self, renderer: &mut Renderer);
}
