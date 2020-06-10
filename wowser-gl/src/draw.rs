use super::enums::{BufferBit, DrawMode};
use super::{get_error_result, GlResult};
use wowser_gl_sys::*;

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
