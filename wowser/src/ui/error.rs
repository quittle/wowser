use std::error::Error;
use wowser_gl as gl;
use wowser_glfw as glfw;
use wowser_macros::DisplayFromDebug;

pub type UiResult = Result<(), UiError>;

#[derive(Debug, DisplayFromDebug)]
pub enum UiError {
    GlError(gl::GlError),
    GlfwError(glfw::GlfwError),
}

impl Error for UiError {}

impl From<UiError> for String {
    fn from(error: UiError) -> String {
        error.to_string()
    }
}

impl From<gl::GlError> for UiError {
    fn from(error: gl::GlError) -> Self {
        UiError::GlError(error)
    }
}

impl From<glfw::GlfwError> for UiError {
    fn from(error: glfw::GlfwError) -> Self {
        UiError::GlfwError(error)
    }
}
