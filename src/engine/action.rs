
use AnyGameScene;
use Engine;

pub enum EngineAction {
    Nothing,
    ToggleFullScreen,
    PopScene,
    PushScene(Box<FnMut(&Engine) -> AnyGameScene>),
    SwitchToScene(Box<FnMut(&Engine) -> AnyGameScene>),
    Quit,
}


impl Default for EngineAction {
    fn default() -> Self {
        EngineAction::Nothing
    }
}
