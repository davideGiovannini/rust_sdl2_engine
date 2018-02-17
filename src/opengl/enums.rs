#![allow(unused)]
use gl::types::*;
use gl;

#[derive(Clone, Copy, Debug)]
pub enum Type {
    UnsignedByte,
    Byte,
    UnsignedShort,
    Short,
    UnsignedInt,
    Int,
    Float,
}

impl Into<GLenum> for Type {
    fn into(self) -> GLenum {
        use self::Type::*;
        match self {
            UnsignedByte => gl::UNSIGNED_BYTE,
            Byte => gl::BYTE,
            UnsignedShort => gl::UNSIGNED_SHORT,
            Short => gl::SHORT,
            UnsignedInt => gl::UNSIGNED_INT,
            Int => gl::INT,
            Float => gl::FLOAT,
        }
    }
}
