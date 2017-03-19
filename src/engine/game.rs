
use {Engine, EngineAction, EngineContext};

use sdl2::event::Event;

pub type AnyGameScene = Box<GameScene>;

pub trait GameScene {
    fn set_up(&mut self);

    fn process_event(&mut self, _: &Event) {}

    fn logic(&mut self, context: EngineContext) -> EngineAction;

    fn render(&mut self, engine: &mut Engine);

    /// Called when another scene has been pushed on the stack
    fn on_pause(&mut self){}
    /// Called when this scene is reactivated.
    fn on_resume(&mut self){}
}
