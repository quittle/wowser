use super::{
    get_error, get_result_if_not_error, initialize_glfw_callbacks, GlfwError, GlfwResult,
    WindowHint,
};
use libc::c_void;
use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr::{self, NonNull};
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
    window: NonNull<GLFWwindow>,
    is_alive: bool,
    pub(crate) window_resize_event: Option<(i32, i32)>,
    pub(crate) window_move_event: Option<(i32, i32)>,
}

impl Window {
    pub fn new(
        width: i32,
        height: i32,
        title: &str,
        share: Option<Window>,
    ) -> Result<Box<Window>, GlfwError> {
        let mut window = create_window(width, height, title, share)?;

        let window_ptr: *mut Window = &mut *window;

        let glfw_window_ptr = window.get_glfw_window_ptr();
        unsafe {
            glfwSetWindowUserPointer(glfw_window_ptr, window_ptr as *mut c_void);
        }
        get_glfw_result()?;

        initialize_glfw_callbacks(&mut window)?;

        Ok(window)
    }

    pub fn get_window_resize_event(&mut self) -> Option<(i32, i32)> {
        self.window_resize_event.take()
    }

    pub fn get_window_move_event(&mut self) -> Option<(i32, i32)> {
        self.window_move_event.take()
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
        match make_context_current(self) {
            GlfwError::NoError => Ok(()),
            err => Err(err),
        }
    }

    pub fn swap_buffers(&self) {
        swap_buffers(self);
    }

    pub fn get_glfw_window_ptr(&self) -> *mut GLFWwindow {
        self.window.as_ptr()
    }

    pub fn should_close(&self) -> Result<bool, GlfwError> {
        window_should_close(self)
    }

    pub fn close(&mut self) -> GlfwResult {
        destroy_window(self)?;
        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

pub fn create_window(
    width: i32,
    height: i32,
    title: &str,
    share: Option<Window>,
) -> Result<Box<Window>, GlfwError> {
    let c_title =
        CString::new(title).expect("Invalid string. Potentially includes null byte characters");
    let share_ptr = match share {
        Some(window) => window.get_glfw_window_ptr(),
        None => ptr::null_mut(),
    };

    let c_title_ptr = c_title.as_ptr();

    let window =
        unsafe { glfwCreateWindow(width, height, c_title_ptr, ptr::null_mut(), share_ptr) };
    get_glfw_result()?;

    if let Some(window) = NonNull::new(window) {
        Ok(Box::new(Window {
            window,
            is_alive: true,
            window_resize_event: None,
            window_move_event: None,
        }))
    } else {
        Err(get_error())
    }
}

fn destroy_window(window: &mut Window) -> GlfwResult {
    if !window.is_alive {
        return Ok(());
    }

    unsafe {
        glfwDestroyWindow(window.get_glfw_window_ptr());
    }

    let result = get_glfw_result();
    if result.is_ok() {
        window.is_alive = false;
    }
    result
}

pub fn set_window_size(window: &Window, width: i32, height: i32) -> GlfwResult {
    unsafe {
        glfwSetWindowSize(window.get_glfw_window_ptr(), width, height);
    }

    get_glfw_result()
}

pub fn set_window_pos(window: &Window, xpos: i32, ypos: i32) -> GlfwResult {
    unsafe {
        glfwSetWindowPos(window.get_glfw_window_ptr(), xpos, ypos);
    }

    get_glfw_result()
}

pub fn make_context_current(window: &Window) -> GlfwError {
    unsafe { glfwMakeContextCurrent(window.get_glfw_window_ptr()) }

    get_error()
}

pub fn swap_buffers(window: &Window) {
    unsafe { glfwSwapBuffers(window.get_glfw_window_ptr()) }
}

pub fn get_window_pos(window: &Window) -> Result<(u32, u32), GlfwError> {
    let mut xpos = 0;
    let mut ypos = 0;
    let xpos_ptr = ptr::addr_of_mut!(xpos);
    let ypos_ptr = ptr::addr_of_mut!(ypos);
    unsafe {
        glfwGetWindowPos(window.get_glfw_window_ptr(), xpos_ptr, ypos_ptr);
    }

    get_glfw_result()?;

    Ok((xpos as u32, ypos as u32))
}

pub fn get_window_size(window: &Window) -> Result<(u32, u32), GlfwError> {
    let mut width = 0;
    let mut height = 0;
    let width_ptr = ptr::addr_of_mut!(width);
    let height_ptr = ptr::addr_of_mut!(height);
    unsafe {
        glfwGetWindowSize(window.get_glfw_window_ptr(), width_ptr, height_ptr);
    }

    get_glfw_result()?;

    Ok((width as u32, height as u32))
}

pub fn poll_events() -> GlfwResult {
    unsafe {
        glfwPollEvents();
    }

    get_glfw_result()
}

pub fn wait_events() -> GlfwResult {
    unsafe {
        glfwWaitEvents();
    }

    get_glfw_result()
}

pub fn get_glfw_result() -> GlfwResult {
    match get_error() {
        GlfwError::NoError => Ok(()),
        err => Err(err),
    }
}

pub fn window_should_close(window: &Window) -> Result<bool, GlfwError> {
    let should_close = unsafe { glfwWindowShouldClose(window.get_glfw_window_ptr()) };

    // Return the error or the actual result of should_close
    get_result_if_not_error(|| should_close != 0)
}

pub fn set_window_hint(hint: WindowHint, value: bool) -> GlfwResult {
    let glfw_hint = hint.into();
    let glfw_value = bool_to_glfw_bool(value);

    unsafe {
        glfwWindowHint(glfw_hint, glfw_value);
    }

    get_glfw_result()
}

pub fn bool_to_glfw_bool(value: bool) -> c_int {
    if value {
        GLFW_TRUE as c_int
    } else {
        GLFW_FALSE as c_int
    }
}
