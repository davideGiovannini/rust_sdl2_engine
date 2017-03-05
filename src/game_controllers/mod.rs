

mod game_controller;
pub use self::game_controller::GameController;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameControllerManager {
    controllers: HashMap<i32, GameController>,
}

impl GameControllerManager {
    pub fn new() -> GameControllerManager {
        unsafe {
            use sdl2_sys as ll;
            ll::SDL_InitSubSystem(ll::SDL_INIT_GAMECONTROLLER | ll::SDL_INIT_HAPTIC);
        };
        GameControllerManager { controllers: GameControllerManager::load_all_connected_devices() }
    }

    fn load_all_connected_devices() -> HashMap<i32, GameController> {
        unsafe {
            use sdl2_sys::joystick::*;
            let num_joysticks = SDL_NumJoysticks();

            let mut map = HashMap::with_capacity(num_joysticks as usize);

            for joystick_index in 0..num_joysticks {
                if let Some(controller) = GameController::from_joystick_index(joystick_index) {
                    map.insert(joystick_index, controller);
                }
            }
            println!("map {:#?}", map);
            map
        }
    }


    pub fn added_controller(&mut self, which: i32) {
        println!("Added controller {:?}", which);
        if let Some(controller) = GameController::from_joystick_index(which) {
            self.controllers.insert(controller.instance_id(), controller);
        }
    }

    pub fn removed_controller(&mut self, which: i32) {
        println!("Disconnected controller {:?}", which);
        if let Some(controller) = self.controllers.remove(&which) {
            self::game_controller::close_controller(controller)
        }
    }

    pub fn snapshot(&self) -> HashMap<i32, GameController> {
        self.controllers.clone()
    }
}
