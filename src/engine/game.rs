use {Engine, EngineAction, EngineContext};

use sdl2::event::Event;
use imgui::Ui;

use resources::PathKey;

pub type AnyGameScene = Box<GameScene>;

pub trait GameScene {
    fn set_up(&mut self);

    #[allow(unused)]
    fn process_event(&mut self, event: &Event) {}

    fn logic(&mut self, context: &EngineContext, engine: &mut Engine, ui: &Ui) -> EngineAction;

    fn render(&mut self, context: &EngineContext, engine: &mut Engine);

    #[allow(unused)]
    fn on_cache_updated(&mut self, engine: &mut Engine, key: PathKey) {}

    /// Called when another scene has been pushed on the stack
    fn on_pause(&mut self) {}
    /// Called when this scene is reactivated.
    fn on_resume(&mut self) {}
}

pub trait FromEngine {
    fn init(engine: &mut Engine) -> Self;
}
