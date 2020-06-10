use std::{error::Error, ffi::CStr, ptr};
use wowser_glfw_sys::*;
use wowser_macros::DisplayFromDebug;

pub type GlfwResult = Result<(), GlfwError>;

#[derive(Debug, DisplayFromDebug)]
pub enum GlfwError {
    NoError,
    NotInitialized(String),
    NoCurrentContext(String),
    InvalidEnum(String),
    InvalidValue(String),
    OutOfMemory(String),
    ApiUnavailable(String),
    VersionUnavailable(String),
    PlatformError(String),
    FormatUnavailable(String),
    NoWindowContext(String),
    UnknownError(String),
}

impl Error for GlfwError {}

impl Into<String> for GlfwError {
    fn into(self) -> String {
        self.to_string()
    }
}

/// Gets the error flag set by GLFW. If no error was set then `GlfwError::NoError`
/// will be returned.
pub fn get_error() -> GlfwError {
    let mut value: *const libc::c_char = ptr::null();

    let err_code = unsafe { glfwGetError(&mut value) } as u32;

    let err_message = if err_code == GLFW_NO_ERROR && !value.is_null() {
        unsafe { CStr::from_ptr(value) }.to_string_lossy().to_string()
    } else {
        String::default()
    };

    match err_code {
        GLFW_NO_ERROR => GlfwError::NoError,
        GLFW_NOT_INITIALIZED => GlfwError::NotInitialized(err_message),
        GLFW_NO_CURRENT_CONTEXT => GlfwError::NoCurrentContext(err_message),
        GLFW_INVALID_ENUM => GlfwError::InvalidEnum(err_message),
        GLFW_INVALID_VALUE => GlfwError::InvalidValue(err_message),
        GLFW_OUT_OF_MEMORY => GlfwError::OutOfMemory(err_message),
        GLFW_API_UNAVAILABLE => GlfwError::ApiUnavailable(err_message),
        GLFW_VERSION_UNAVAILABLE => GlfwError::VersionUnavailable(err_message),
        GLFW_PLATFORM_ERROR => GlfwError::PlatformError(err_message),
        GLFW_FORMAT_UNAVAILABLE => GlfwError::FormatUnavailable(err_message),
        GLFW_NO_WINDOW_CONTEXT => GlfwError::NoWindowContext(err_message),
        _ => GlfwError::UnknownError(err_message),
    }
}

/// Like `get_error`, except returns an Ok if `GlfwError::NoError` or
/// returns Err(GlfwError) if not.
pub fn get_error_result() -> GlfwResult {
    match get_error() {
        GlfwError::NoError => Ok(()),
        err => Err(err),
    }
}
