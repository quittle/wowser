use std::ffi::CString;
use std::ptr;

use super::{get_error, GlfwError, GlfwResult};
use wowser_glfw_sys::*;

pub type ErrorCallback = unsafe extern "C" fn(i32, *const i8);

pub fn set_error_callback(callback: Option<ErrorCallback>) {
    unsafe {
        glfwSetErrorCallback(callback);
    }
}

pub fn init() -> GlfwResult {
    let init_result = unsafe { glfwInit() };
    let successful = init_result == GLFW_TRUE as i32;
    if successful {
        Ok(())
    } else {
        Err(get_error())
    }
}

pub fn terminate() -> GlfwResult {
    unsafe {
        glfwTerminate();
    }

    match get_error() {
        GlfwError::NoError => Ok(()),
        err => Err(err),
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

    pub fn set_window_size(&self, width: i32, height: i32) -> GlfwResult {
        set_window_size(self, width, height)
    }

    pub fn set_window_pos(&self, xpos: i32, ypos: i32) -> GlfwResult {
        set_window_pos(self, xpos, ypos)
    }

    pub fn get_window_bounds(&self) -> Result<(u32, u32, u32, u32), GlfwError> {
        let (width, height) = get_window_size(self)?;
        let (x, y) = get_window_pos(self)?;
        Ok((x, y, width, height))
    }

    pub fn make_context_current(&self) -> GlfwResult {
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

    let c_title_ptr = c_title.as_ptr();

    let window =
        unsafe { glfwCreateWindow(width, height, c_title_ptr, ptr::null_mut(), share_ptr) };

    if window.is_null() {
        Err(get_error())
    } else {
        Ok(Window { window })
    }
}

pub fn set_window_size(window: &Window, width: i32, height: i32) -> GlfwResult {
    unsafe {
        glfwSetWindowSize(window.window, width, height);
    }

    match get_error() {
        GlfwError::NoError => Ok(()),
        err => Err(err),
    }
}

pub fn set_window_pos(window: &Window, xpos: i32, ypos: i32) -> GlfwResult {
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

pub fn get_window_pos(window: &Window) -> Result<(u32, u32), GlfwError> {
    let mut xpos = 0;
    let mut ypos = 0;
    let xpos_ptr = ptr::addr_of_mut!(xpos);
    let ypos_ptr = ptr::addr_of_mut!(ypos);
    unsafe {
        glfwGetWindowPos(window.window, xpos_ptr, ypos_ptr);
    }

    match get_error() {
        GlfwError::NoError => Ok((xpos as u32, ypos as u32)),
        err => Err(err),
    }
}

pub fn get_window_size(window: &Window) -> Result<(u32, u32), GlfwError> {
    let mut width = 0;
    let mut height = 0;
    let width_ptr = ptr::addr_of_mut!(width);
    let height_ptr = ptr::addr_of_mut!(height);
    unsafe {
        glfwGetWindowSize(window.window, width_ptr, height_ptr);
    }

    match get_error() {
        GlfwError::NoError => Ok((width as u32, height as u32)),
        err => Err(err),
    }
}
