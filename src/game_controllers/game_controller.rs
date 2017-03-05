#![allow(unused)]

use sdl2_sys::controller::*;
use sdl2_sys::joystick::*;
use sdl2_sys::haptic::*;

use sdl2::controller::{Axis, Button};

#[derive(Clone, Debug)]
pub struct GameController {
    raw_game_controller: *mut SDL_GameController,
    raw_joystick: *mut SDL_Joystick,
    joystick_id: i32,
    haptic: Option<Haptic>,
}

impl GameController {
    pub fn from_joystick_index(index: i32) -> Option<GameController> {
        return unsafe {
            let joystick = SDL_JoystickOpen(index);
            if SDL_IsGameController(index) != 0 {
                let controller = SDL_GameControllerOpen(index);
                if controller.is_null() {
                    None
                } else {
                    let joystick = SDL_GameControllerGetJoystick(controller);

                    let haptic = if SDL_JoystickIsHaptic(joystick) == 1 {
                        Some(Haptic::from_joystick(joystick))
                    } else {
                        None
                    };
                    Some(GameController {
                        raw_game_controller: controller,
                        raw_joystick: joystick,
                        joystick_id: SDL_JoystickInstanceID(joystick),
                        haptic: haptic,
                    })
                }
            } else {
                None
            }
        };
    }

    pub fn instance_id(&self) -> i32 {
        self.joystick_id
    }

    pub fn play_rumble(&self, strenght: f32, duration: u32) {
        if let Some(ref haptic) = self.haptic {
            haptic.play(strenght, duration)
        }
    }

    /// Get the position of the given `axis`
    pub fn axis(&self, axis: Axis) -> i16 {
        let axis = axis as SDL_GameControllerAxis;

        unsafe { SDL_GameControllerGetAxis(self.raw_game_controller, axis) }
    }

    /// Returns `true` if `button` is pressed.
    pub fn button(&self, button: Button) -> bool {
        let button = button as SDL_GameControllerButton;

        unsafe { SDL_GameControllerGetButton(self.raw_game_controller, button) != 0 }
    }
}

unsafe impl Send for GameController {
    // TODO make sure this does not cause problems
}

#[derive(Clone, Debug)]
struct Haptic {
    raw: *mut SDL_Haptic,
}


impl Haptic {
    fn from_joystick(joystick: *mut SDL_Joystick) -> Haptic {
        unsafe {
            use sdl2_sys::haptic::*;

            let haptic = SDL_HapticOpenFromJoystick(joystick);
            SDL_HapticRumbleInit(haptic);

            Haptic { raw: haptic }
        }
    }

    fn play(&self, strenght: f32, duration: u32) {
        unsafe {
            use sdl2_sys::haptic::*;
            SDL_HapticRumblePlay(self.raw, strenght, duration)
        };
    }
}

pub fn close_controller(controller: GameController) {
    unsafe {
        use sdl2_sys::joystick::*;
        use sdl2_sys::haptic::*;

        if let Some(haptic) = controller.haptic {
            SDL_HapticClose(haptic.raw);
        }
        SDL_GameControllerClose(controller.raw_game_controller);
        SDL_JoystickClose(controller.raw_joystick)
    }
}
