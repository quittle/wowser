use libc::{c_char, c_int};
use std::ffi::CString;
use std::ptr;

pub fn glfw_init() -> bool {
    unsafe { glfwInit() != 0 }
}

pub fn glfw_terminate() {
    unsafe {
        glfwTerminate();
    }
}

pub fn glfw_create_window(width: i32, height: i32, title: &str) {
    unsafe {
        glfwCreateWindow(
            width,
            height,
            CString::new(title).expect("Invalid string").as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }
}

#[repr(C)]
pub struct GLFWmonitor {}

#[repr(C)]
pub struct GLFWwindow {}

#[link(name = "glfw3")]
extern "C" {
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate() -> ();
    pub fn glfwCreateWindow(
        width: c_int,
        height: c_int,
        title: *const c_char,
        monitor: *mut GLFWmonitor,
        share: *mut GLFWwindow,
    ) -> *mut GLFWwindow;
}
