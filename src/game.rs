
use sdl2::render::Renderer;
use sdl2::ttf::Sdl2TtfContext;
use EngineContext;
use EngineAction;

use sdl2::event::Event;

pub trait Game {
    fn init(renderer: &Renderer, ttf_context: Sdl2TtfContext) -> Self;

    fn set_up(&mut self);

    fn process_event(&mut self, _: &Event){}

    fn logic(&mut self, context: EngineContext) -> EngineAction;

    fn render(&mut self, renderer: &mut Renderer);
}
