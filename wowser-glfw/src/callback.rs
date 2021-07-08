use super::ptr_holder::PtrHolder;
use crate::{get_glfw_result, GlfwResult, Window};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use wowser_glfw_sys::*;

type CallbackMap = HashMap<PtrHolder, fn(i32, i32)>;

lazy_static! {
    static ref WINDOW_RESIZE_CALLBACKS_MAP: Mutex<CallbackMap> = Mutex::new(HashMap::new());
}

unsafe extern "C" fn unsafe_on_window_resize_callback(
    window: *mut GLFWwindow,
    width: i32,
    height: i32,
) {
    on_window_resize_callback(window, width, height);
}

fn on_window_resize_callback(window: *mut GLFWwindow, width: i32, height: i32) {
    let map = WINDOW_RESIZE_CALLBACKS_MAP.lock().unwrap();
    if let Some(callback) = map.get(&PtrHolder::new(window)) {
        callback(width, height);
    }
}

pub fn set_window_size_callback(window: &Window, callback: Option<fn(i32, i32)>) -> GlfwResult {
    let mut map = WINDOW_RESIZE_CALLBACKS_MAP.lock()?;
    let ptr_holder = PtrHolder::new(window.get_glfw_window_ptr());
    if let Some(callback) = callback {
        map.insert(ptr_holder, callback);
    } else {
        map.remove(&ptr_holder);
    }

    unsafe {
        glfwSetWindowSizeCallback(
            window.get_glfw_window_ptr(),
            // Can't be set outside of this statement or it results in a type error
            if callback.is_some() {
                Some(unsafe_on_window_resize_callback)
            } else {
                None
            },
        );
    }

    get_glfw_result()
}
