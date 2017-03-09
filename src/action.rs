

pub enum EngineAction{

  Nothing, ToggleFullScreen, Quit, SwitchTo
}


impl Default for EngineAction{
  fn default() -> Self{
    EngineAction::Nothing
  }
}
