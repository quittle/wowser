use std::error::Error;
use wowser_gl_sys::*;
use wowser_macros::DisplayFromDebug;

pub type GlResult = Result<(), GlError>;

/// Represents [GL errors](https://www.khronos.org/opengl/wiki/OpenGL_Error).
#[derive(Debug, DisplayFromDebug)]
pub enum GlError {
    NoError,
    Error(Vec<GlSingleError>),
}

impl Error for GlError {}

impl Into<String> for GlError {
    fn into(self) -> String {
        self.to_string()
    }
}

#[derive(Debug, DisplayFromDebug)]
pub enum GlSingleError {
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    StackOverflow,
    StackUnderflow,
    OutOfMemory,
    InvalidFramebufferOperation,
    ContextLost,
    TableTooLarge,
    UnknownError(u32),
}

/// Gets the error flag set by GLFW. If no error was set then `GlError::NoError`
/// will be returned.
pub fn get_error() -> GlError {
    let mut errors = vec![];
    loop {
        let error_code = unsafe { glGetError() };

        let error = match error_code {
            GL_NO_ERROR => break,
            GL_INVALID_ENUM => GlSingleError::InvalidEnum,
            GL_INVALID_VALUE => GlSingleError::InvalidValue,
            GL_INVALID_OPERATION => GlSingleError::InvalidOperation,
            GL_STACK_OVERFLOW => GlSingleError::StackOverflow,
            GL_STACK_UNDERFLOW => GlSingleError::StackUnderflow,
            GL_OUT_OF_MEMORY => GlSingleError::OutOfMemory,
            GL_INVALID_FRAMEBUFFER_OPERATION => GlSingleError::InvalidFramebufferOperation,
            GL_CONTEXT_LOST => GlSingleError::ContextLost,
            GL_TABLE_TOO_LARGE => GlSingleError::TableTooLarge,
            _ => GlSingleError::UnknownError(error_code),
        };

        errors.push(error);
    }

    if errors.is_empty() {
        GlError::NoError
    } else {
        GlError::Error(errors)
    }
}

/// Like `get_error`, except returns an Ok if `GlError::NoError` or
/// returns Err(GlError) if not.
pub fn get_error_result() -> GlResult {
    match get_error() {
        GlError::NoError => Ok(()),
        err => Err(err),
    }
}
