use crate::{get_glfw_result, GlfwResult, Window};
use libc::c_void;
use wowser_glfw_sys::*;

fn get_window_from_glfw_user_pointer(window: *mut GLFWwindow) -> Option<&'static mut Window> {
    unsafe {
        let ptr: *mut c_void = glfwGetWindowUserPointer(window);
        if ptr.is_null() {
            return None;
        }
        let wrapper_window_ptr: *mut Window = ptr as *mut Window;
        Some(&mut *wrapper_window_ptr)
    }
}

unsafe extern "C" fn unsafe_on_window_resize_callback(
    window: *mut GLFWwindow,
    width: i32,
    height: i32,
) {
    on_window_resize_callback(window, width, height);
}

fn on_window_resize_callback(window: *mut GLFWwindow, width: i32, height: i32) {
    if let Some(window) = get_window_from_glfw_user_pointer(window) {
        window.window_resize_event = Some((width, height));
    }
}

unsafe extern "C" fn unsafe_on_window_move_callback(window: *mut GLFWwindow, x: i32, y: i32) {
    on_window_move_callback(window, x, y);
}

fn on_window_move_callback(window: *mut GLFWwindow, x: i32, y: i32) {
    if let Some(window) = get_window_from_glfw_user_pointer(window) {
        window.window_move_event = Some((x, y));
    }
}

pub(super) fn initialize_glfw_callbacks(window: &mut Window) -> GlfwResult {
    let glfw_window_ptr = window.get_glfw_window_ptr();
    unsafe { glfwSetWindowSizeCallback(glfw_window_ptr, Some(unsafe_on_window_resize_callback)) };
    unsafe { glfwSetWindowPosCallback(glfw_window_ptr, Some(unsafe_on_window_move_callback)) };

    get_glfw_result()
}
