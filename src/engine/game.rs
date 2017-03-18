
use {Engine, EngineAction, EngineContext};

use sdl2::event::Event;

pub trait Game {
    fn init(engine: &Engine) -> Self;

    fn set_up(&mut self);

    fn process_event(&mut self, _: &Event) {}

    fn logic(&mut self, context: EngineContext) -> EngineAction;

    fn render(&mut self, engine: &mut Engine);
}
