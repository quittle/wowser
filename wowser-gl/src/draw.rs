use super::{get_error_result, GlError};
use wowser_gl_sys::*;

pub fn bitmap(
    width: i32,
    height: i32,
    x_orig: f32,
    y_orig: f32,
    x_move: f32,
    y_move: f32,
    bitmap: &[u8],
) -> Result<(), GlError> {
    let bitmap_ptr = bitmap.as_ptr();

    unsafe {
        glBitmap(width, height, x_orig, y_orig, x_move, y_move, bitmap_ptr);
    }

    get_error_result()
}
