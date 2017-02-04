
use sdl2::render::Renderer;
use EngineContext;

pub trait Game {
    fn init(renderer: &Renderer) -> Self;

    fn set_up(&mut self);

    fn logic(&mut self, context: EngineContext);

    fn render(&mut self, renderer: &mut Renderer);
}
