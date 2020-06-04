use std::ffi::CString;
use std::ptr;

use super::{get_error, GlfwError};
use wowser_glfw_sys::*;

pub type ErrorCallback = unsafe extern "C" fn(i32, *const i8);

pub fn set_error_callback(callback: Option<ErrorCallback>) {
    unsafe {
        glfwSetErrorCallback(callback);
    }
}

pub fn init() -> Result<(), GlfwError> {
    let init_result = unsafe { glfwInit() };
    let successful = init_result == GLFW_TRUE as i32;
    if successful {
        Ok(())
    } else {
        Err(get_error())
    }
}

pub fn terminate() {
    unsafe {
        glfwTerminate();
    }
}

pub struct Window {
    window: *mut GLFWwindow,
}

impl Window {
    pub fn new(
        width: i32,
        height: i32,
        title: &str,
        share: Option<Window>,
    ) -> Result<Window, GlfwError> {
        create_window(width, height, title, share)
    }

    pub fn set_window_size(&self, width: i32, height: i32) -> Result<(), GlfwError> {
        set_window_size(self, width, height)
    }

    pub fn set_window_pos(&self, xpos: i32, ypos: i32) -> Result<(), GlfwError> {
        set_window_pos(self, xpos, ypos)
    }

    pub fn make_context_current(&self) -> Result<(), GlfwError> {
        match make_context_current(&self) {
            GlfwError::NoError => Ok(()),
            err => Err(err),
        }
    }

    pub fn swap_buffers(&self) {
        swap_buffers(self);
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        println!("Destroying window");
        destroy_window(self);
    }
}

pub fn create_window(
    width: i32,
    height: i32,
    title: &str,
    share: Option<Window>,
) -> Result<Window, GlfwError> {
    let c_title = CString::new(title).expect("Invalid string");
    let share_ptr = match share {
        Some(window) => window.window,
        None => ptr::null_mut(),
    };

    let window =
        unsafe { glfwCreateWindow(width, height, c_title.as_ptr(), ptr::null_mut(), share_ptr) };

    if window.is_null() {
        Err(get_error())
    } else {
        Ok(Window { window })
    }
}

pub fn set_window_size(window: &Window, width: i32, height: i32) -> Result<(), GlfwError> {
    unsafe {
        glfwSetWindowSize(window.window, width, height);
    }

    match get_error() {
        GlfwError::NoError => Ok(()),
        err => Err(err),
    }
}

pub fn set_window_pos(window: &Window, xpos: i32, ypos: i32) -> Result<(), GlfwError> {
    unsafe {
        glfwSetWindowPos(window.window, xpos, ypos);
    }

    match get_error() {
        GlfwError::NoError => Ok(()),
        err => Err(err),
    }
}

pub fn destroy_window(window: &mut Window) {
    unsafe { glfwDestroyWindow(window.window) }
    window.window = ptr::null_mut();
}

pub fn make_context_current(window: &Window) -> GlfwError {
    unsafe { glfwMakeContextCurrent(window.window) }

    get_error()
}

pub fn swap_buffers(window: &Window) {
    unsafe { glfwSwapBuffers(window.window) }
}
