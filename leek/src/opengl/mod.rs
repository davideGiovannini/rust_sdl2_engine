use sdl2;

use gl::types::GLfloat;

use std::mem;

mod enums;
mod shader;
mod state;
mod texture;
mod vertex_attrib;
mod vertex_buffer;
pub use self::enums::*;
pub use self::shader::*;
pub use self::state::*;
pub use self::texture::*;
pub use self::vertex_attrib::*;
pub use self::vertex_buffer::*;

pub const GL_FLOAT_SIZE: usize = mem::size_of::<GLfloat>();

pub fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub fn log_messages() {
    unsafe {
        use gl;
        use std::ffi;

        let num_msgs = 1;
        let mut max_msg_len = 0;
        gl::GetIntegerv(gl::MAX_DEBUG_MESSAGE_LENGTH, &mut max_msg_len);

        //            std::vector<GLchar> msgData(numMsgs * maxMsgLen);
        let mut msg_data: Vec<u8> = Vec::with_capacity(num_msgs * max_msg_len as usize);
        let mut sources = 0;
        let mut types = 0;
        let mut severities = 0;
        let mut ids = 0;
        let mut lengths = 0;

        //            GLuint numFound = glGetDebugMessageLog(numMsgs, msgs.size(), &sources[0], &types[0], &ids[0], &severities[0], &lengths[0], &msgData[0]);

        let msg_count = gl::GetDebugMessageLog(
            num_msgs as u32,
            256,
            &mut sources,
            &mut types,
            &mut ids,
            &mut severities,
            &mut lengths,
            msg_data.as_mut_ptr() as *mut i8,
        );

        if msg_count > 0 {
            let msg_source = match sources {
                gl::DEBUG_SOURCE_API => "API".to_string(),
                gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW SYSTEM".to_string(),
                gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER COMPILER".to_string(),
                gl::DEBUG_SOURCE_THIRD_PARTY => "THIRDY PARTY".to_string(),
                gl::DEBUG_SOURCE_APPLICATION => "APPLICATION".to_string(),
                gl::DEBUG_SOURCE_OTHER => "OTHER".to_string(),
                _ => format!("[{}]", sources),
            };

            let msg_type = match types {
                gl::DEBUG_TYPE_ERROR => "ERROR".to_string(),
                gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED BEHAVIOR".to_string(),
                gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED BEHAVIOR".to_string(),
                gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY".to_string(),
                gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE".to_string(),
                gl::DEBUG_TYPE_MARKER => "MARKER".to_string(),
                gl::DEBUG_TYPE_PUSH_GROUP => "PUSH GROUP".to_string(),
                gl::DEBUG_TYPE_POP_GROUP => "POP GROUP".to_string(),
                gl::DEBUG_TYPE_OTHER => "OTHER".to_string(),
                _ => format!("[{}]", types),
            };

            let msg_severity = match severities {
                gl::DEBUG_SEVERITY_HIGH => "HIGH".to_string(),
                gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM".to_string(),
                gl::DEBUG_SEVERITY_LOW => "LOW".to_string(),
                gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION".to_string(),
                _ => format!("[{}]", severities),
            };

            println!(
                "Message[{}] from {} of type {} (Severity {})",
                ids, msg_source, msg_type, msg_severity
            );
            msg_data.set_len(lengths as usize - 1);
            println!("\t{:?}", ffi::CString::from_vec_unchecked(msg_data));
        }
    }
}
