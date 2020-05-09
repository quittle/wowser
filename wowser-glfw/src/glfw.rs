extern crate wowser_glfw_sys;

use std::ffi::CString;
use std::ptr;

use wowser_glfw_sys::*;

pub fn glfw_init() -> bool {
    unsafe { glfwInit() != 0 }
}

pub fn glfw_terminate() {
    unsafe {
        glfwTerminate();
    }
}

pub fn glfw_create_window(width: i32, height: i32, title: &str) {
    let c_title = CString::new(title).expect("Invalid string").as_ptr();
    unsafe {
        glfwCreateWindow(width, height, c_title, ptr::null_mut(), ptr::null_mut());
    }
}
