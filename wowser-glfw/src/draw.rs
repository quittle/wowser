use super::{get_error_result, GlfwError};
use wowser_glfw_sys::*;

pub fn bitmap(
    width: i32,
    height: i32,
    x_orig: f32,
    y_orig: f32,
    x_move: f32,
    y_move: f32,
    bitmap: &[u8],
) -> Result<(), GlfwError> {
    unsafe {
        glBitmap(width, height, x_orig, y_orig, x_move, y_move, bitmap.as_ptr());
    }

    let err_code = unsafe { glGetError() } as u32;
    println!("ERR CODE {}", err_code);

    get_error_result()
}
