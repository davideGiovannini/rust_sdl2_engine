use imgui::ImGui;
use imgui::ImGuiKey;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::EventPump;
use std::os::raw::c_void;

pub fn configure_keys(imgui: &mut ImGui) {
    imgui.set_imgui_key(ImGuiKey::Tab, 0);
    imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
    imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
    imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
    imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
    imgui.set_imgui_key(ImGuiKey::PageUp, 5);
    imgui.set_imgui_key(ImGuiKey::PageDown, 6);
    imgui.set_imgui_key(ImGuiKey::Home, 7);
    imgui.set_imgui_key(ImGuiKey::End, 8);
    imgui.set_imgui_key(ImGuiKey::Delete, 9);
    imgui.set_imgui_key(ImGuiKey::Backspace, 10);
    imgui.set_imgui_key(ImGuiKey::Enter, 11);
    imgui.set_imgui_key(ImGuiKey::Escape, 12);
    imgui.set_imgui_key(ImGuiKey::A, 13);
    imgui.set_imgui_key(ImGuiKey::C, 14);
    imgui.set_imgui_key(ImGuiKey::V, 15);
    imgui.set_imgui_key(ImGuiKey::X, 16);
    imgui.set_imgui_key(ImGuiKey::Y, 17);
    imgui.set_imgui_key(ImGuiKey::Z, 18);

    // Register clipboard functions
    imgui.io_mut().set_clipboard_text_fn = Some(set_clipboard);
    imgui.io_mut().get_clipboard_text_fn = Some(get_clipboard);
}

extern "C" fn set_clipboard(_: *mut c_void, data: *const i8) {
    unsafe {
        use sdl2::sys::SDL_SetClipboardText;
        SDL_SetClipboardText(data);
    }
}

extern "C" fn get_clipboard(_: *mut c_void) -> *const i8 {
    unsafe {
        use sdl2::sys::SDL_GetClipboardText;
        SDL_GetClipboardText()
    }
}

pub fn process_event_state(imgui: &mut ImGui, event_pump: &EventPump) {
    let mouse = event_pump.mouse_state();
    let keyboard = event_pump.keyboard_state();

    use sdl2::keyboard::Scancode::*;
    imgui.set_key(0, keyboard.is_scancode_pressed(Tab));
    imgui.set_key(1, keyboard.is_scancode_pressed(Left));
    imgui.set_key(2, keyboard.is_scancode_pressed(Right));
    imgui.set_key(3, keyboard.is_scancode_pressed(Up));
    imgui.set_key(4, keyboard.is_scancode_pressed(Down));
    imgui.set_key(5, keyboard.is_scancode_pressed(PageUp));
    imgui.set_key(6, keyboard.is_scancode_pressed(PageDown));
    imgui.set_key(7, keyboard.is_scancode_pressed(Home));
    imgui.set_key(8, keyboard.is_scancode_pressed(End));
    imgui.set_key(9, keyboard.is_scancode_pressed(Delete));
    imgui.set_key(10, keyboard.is_scancode_pressed(Backspace));
    imgui.set_key(11, keyboard.is_scancode_pressed(Return));
    imgui.set_key(12, keyboard.is_scancode_pressed(Escape));
    imgui.set_key(13, keyboard.is_scancode_pressed(A));
    imgui.set_key(14, keyboard.is_scancode_pressed(C));
    imgui.set_key(15, keyboard.is_scancode_pressed(V));
    imgui.set_key(16, keyboard.is_scancode_pressed(X));
    imgui.set_key(17, keyboard.is_scancode_pressed(Y));
    imgui.set_key(18, keyboard.is_scancode_pressed(Z));
    imgui.set_key_ctrl(keyboard.is_scancode_pressed(LCtrl) || keyboard.is_scancode_pressed(RCtrl));
    imgui.set_key_shift(
        keyboard.is_scancode_pressed(LShift) || keyboard.is_scancode_pressed(RShift),
    );
    imgui.set_key_alt(keyboard.is_scancode_pressed(LAlt) || keyboard.is_scancode_pressed(RAlt));

    let scale = imgui.display_framebuffer_scale();
    imgui.set_mouse_pos(mouse.x() as f32 / scale.0, mouse.y() as f32 / scale.1);
    imgui.set_mouse_down(&[
        mouse.is_mouse_button_pressed(MouseButton::Left),
        mouse.is_mouse_button_pressed(MouseButton::Right),
        mouse.is_mouse_button_pressed(MouseButton::Middle),
        false,
        false,
    ]);
}

pub fn process_event(imgui: &mut ImGui, event: &Event) {
    match *event {
        Event::TextInput { ref text, .. } => for c in text.chars() {
            imgui.add_input_character(c)
        },
        Event::MouseWheel { y, .. } => imgui.set_mouse_wheel(y as f32 / 2.0),
        _ => {}
    }
}
