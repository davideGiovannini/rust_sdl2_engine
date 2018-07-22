#![allow(unused)]

use sdl2::sys;
use sdl2::sys::SDL_GameController;
use sdl2::sys::SDL_GameControllerAxis;
use sdl2::sys::SDL_GameControllerButton;
use sdl2::sys::SDL_GameControllerClose;
use sdl2::sys::SDL_GameControllerGetAxis;
use sdl2::sys::SDL_GameControllerGetButton;
use sdl2::sys::SDL_GameControllerGetJoystick;
use sdl2::sys::SDL_GameControllerOpen;
use sdl2::sys::SDL_Haptic;
use sdl2::sys::SDL_HapticClose;
use sdl2::sys::SDL_HapticOpenFromJoystick;
use sdl2::sys::SDL_HapticRumbleInit;
use sdl2::sys::SDL_HapticRumblePlay;
use sdl2::sys::SDL_IsGameController;
use sdl2::sys::SDL_Joystick;
use sdl2::sys::SDL_JoystickClose;
use sdl2::sys::SDL_JoystickInstanceID;
use sdl2::sys::SDL_JoystickIsHaptic;
use sdl2::sys::SDL_JoystickOpen;
use sdl2::sys::SDL_NumJoysticks;
use sdl2::sys::SDL_bool;

use sdl2::controller::{Axis, Button};

#[derive(Clone, Debug)]
pub struct GameController {
    raw_game_controller: *mut SDL_GameController,
    raw_joystick: *mut SDL_Joystick,
    joystick_id: u32,
    haptic: Option<Haptic>,
}

impl GameController {
    pub fn from_joystick_index(index: u32) -> Option<GameController> {
        unsafe {
            // TODO check that cast is ok
            let joystick = SDL_JoystickOpen(index as i32);
            if let SDL_bool::SDL_TRUE = SDL_IsGameController(index as i32) {
                let controller = SDL_GameControllerOpen(index as i32);
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
                        joystick_id: SDL_JoystickInstanceID(joystick) as u32, // TODO check cast
                        haptic,
                    })
                }
            } else {
                None
            }
        }
    }

    pub fn instance_id(&self) -> u32 {
        self.joystick_id
    }

    pub fn play_rumble(&self, strenght: f32, duration: u32) {
        if let Some(ref haptic) = self.haptic {
            haptic.play(strenght, duration)
        }
    }

    /// Get the position of the given `axis`
    pub fn axis(&self, axis: Axis) -> i16 {
        let raw_axis = match axis {
            Axis::LeftX => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_LEFTX,
            Axis::LeftY => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_LEFTY,
            Axis::RightX => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_RIGHTX,
            Axis::RightY => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_RIGHTY,
            Axis::TriggerLeft => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_TRIGGERLEFT,
            Axis::TriggerRight => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
        };

        unsafe { SDL_GameControllerGetAxis(self.raw_game_controller, raw_axis) }
    }

    /// Returns `true` if `button` is pressed.
    pub fn button(&self, button: Button) -> bool {
        let raw_button = match button {
            Button::A => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_A,
            Button::B => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_B,
            Button::X => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_X,
            Button::Y => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_Y,
            Button::Back => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_BACK,
            Button::Guide => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_GUIDE,
            Button::Start => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_START,
            Button::LeftStick => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_LEFTSTICK,
            Button::RightStick => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_RIGHTSTICK,
            Button::LeftShoulder => {
                sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_LEFTSHOULDER
            }
            Button::RightShoulder => {
                sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER
            }
            Button::DPadUp => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_UP,
            Button::DPadDown => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_DOWN,
            Button::DPadLeft => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_LEFT,
            Button::DPadRight => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
        };

        unsafe { SDL_GameControllerGetButton(self.raw_game_controller, raw_button) != 0 }
    }
}

unsafe impl Send for GameController {
    // TODO make sure this does not cause problems
}
unsafe impl Sync for GameController {
    // TODO make sure this does not cause problems
}
#[derive(Clone, Debug)]
struct Haptic {
    raw: *mut SDL_Haptic,
}

impl Haptic {
    fn from_joystick(joystick: *mut SDL_Joystick) -> Haptic {
        unsafe {
            //            use sdl2::sys::haptic::*;

            let haptic = SDL_HapticOpenFromJoystick(joystick);
            SDL_HapticRumbleInit(haptic);

            Haptic { raw: haptic }
        }
    }

    fn play(&self, strenght: f32, duration: u32) {
        unsafe {
            //            use sdl2::sys::haptic::*;
            SDL_HapticRumblePlay(self.raw, strenght, duration)
        };
    }
}

pub fn close_controller(controller: GameController) {
    unsafe {
        //        use sdl2::sys::joystick::*;
        //        use sdl2::sys::haptic::*;

        if let Some(haptic) = controller.haptic {
            SDL_HapticClose(haptic.raw);
        }
        SDL_GameControllerClose(controller.raw_game_controller);
        SDL_JoystickClose(controller.raw_joystick)
    }
}
