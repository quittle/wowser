use std::ffi::c_void;

use super::enums::{BufferBit, DrawMode, Format, PixelDataType};
use super::{get_error_result, GlResult};
use wowser_gl_sys::*;

/// Draws pixels from the bottom-left corner to the top-right corner of the screen.
pub fn draw_pixels(
    width: usize,
    height: usize,
    format: Format,
    pixel_data_type: PixelDataType,
    data: &[u8],
) -> GlResult {
    debug_assert_eq!(data.len(), width * height * format.get_stride() as usize);

    let data_ptr = data.as_ptr() as *const c_void;
    unsafe {
        glDrawPixels(
            width as i32,
            height as i32,
            format.into(),
            pixel_data_type.into(),
            data_ptr,
        );
    }
    get_error_result()
}

pub fn bitmap(
    width: i32,
    height: i32,
    x_orig: f32,
    y_orig: f32,
    x_move: f32,
    y_move: f32,
    bitmap: &[u8],
) -> GlResult {
    let bitmap_ptr = bitmap.as_ptr();

    unsafe {
        glBitmap(width, height, x_orig, y_orig, x_move, y_move, bitmap_ptr);
    }

    get_error_result()
}

pub fn vertex_2i(x: i32, y: i32) {
    unsafe {
        glVertex2i(x, y);
    }
}

pub fn vertex_2f(x: f32, y: f32) {
    unsafe {
        glVertex2f(x, y);
    }
}

pub fn begin(mode: DrawMode) {
    let gl_mode = mode.into();

    unsafe {
        glBegin(gl_mode);
    }
}

pub fn end() -> GlResult {
    unsafe {
        glEnd();
    }

    get_error_result()
}

pub fn flush() -> GlResult {
    unsafe {
        glFlush();
    }

    get_error_result()
}

pub fn clear(mask: &[BufferBit]) -> GlResult {
    let mut gl_mask: GLbitfield = 0;
    for m in mask {
        gl_mask |= m.as_ref();
    }

    unsafe {
        glClear(gl_mask);
    }

    get_error_result()
}
