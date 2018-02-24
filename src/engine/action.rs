use AnyGameScene;
use Engine;

pub enum EngineAction {
    Nothing,
    ToggleFullScreen,
    PopScene,
    PushScene(Box<FnMut(&mut Engine) -> AnyGameScene>),
    SwitchToScene(Box<FnMut(&mut Engine) -> AnyGameScene>),
    Quit,
}

impl Default for EngineAction {
    fn default() -> Self {
        EngineAction::Nothing
    }
}
