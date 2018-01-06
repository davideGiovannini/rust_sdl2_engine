

mod game_controller;
pub use self::game_controller::GameController;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct GameControllerManager {
    controllers: HashMap<u32, GameController>,
}

impl GameControllerManager {
    pub fn new() -> GameControllerManager {
        unsafe {
            use sdl2::sys as ll;
            ll::SDL_InitSubSystem(ll::SDL_INIT_GAMECONTROLLER | ll::SDL_INIT_HAPTIC);
        };
        GameControllerManager { controllers: GameControllerManager::load_all_connected_devices() }
    }

    fn load_all_connected_devices() -> HashMap<u32, GameController> {
        unsafe {
            use sdl2::sys::SDL_NumJoysticks;

            // TODO check conversion
            let num_joysticks: u32 = SDL_NumJoysticks() as u32;

            let mut map = HashMap::with_capacity(num_joysticks as usize);

            for joystick_index in 0..num_joysticks {
                if let Some(controller) = GameController::from_joystick_index(joystick_index) {
                    map.insert(joystick_index, controller);
                }
            }
            #[cfg(debug_assertions)]
            println!("map {:#?}", map);
            map
        }
    }


    pub fn added_controller(&mut self, which: u32) {
        println!("Added controller {:?}", which);
        if let Some(controller) = GameController::from_joystick_index(which) {
            self.controllers.insert(controller.instance_id(), controller);
        }
    }

    pub fn removed_controller(&mut self, which: u32) {
        println!("Disconnected controller {:?}", which);
        if let Some(controller) = self.controllers.remove(&which) {
            self::game_controller::close_controller(controller)
        }
    }

    pub fn snapshot(&self) -> HashMap<u32, GameController> {
        self.controllers.clone()
    }
}
