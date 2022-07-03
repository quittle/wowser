use std::{ffi::CStr, ptr::addr_of_mut};

use crate::{GlError, GlSingleError, StateFloat, StateString};

use super::{get_error_result, Alignment, AlignmentValue, Capability, GlResult, MatrixMode};
use wowser_gl_sys::*;

pub fn point_size(size: f32) -> GlResult {
    unsafe {
        glPointSize(size);
    }
    get_error_result()
}

pub fn color_3f(red: f32, green: f32, blue: f32) {
    unsafe {
        glColor3f(red, green, blue);
    }
}

pub fn color_4ub(red: u8, green: u8, blue: u8, alpha: u8) {
    unsafe {
        glColor4ub(red, green, blue, alpha);
    }
}

pub fn color_4f(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        glColor4f(red, green, blue, alpha);
    }
}

pub fn line_width(width: f32) -> GlResult {
    unsafe {
        glLineWidth(width);
    }
    get_error_result()
}

pub fn pixel_zoom(xfactor: f32, yfactor: f32) -> GlResult {
    unsafe {
        glPixelZoom(xfactor, yfactor);
    }

    get_error_result()
}

pub fn raster_pos_2i(x: i32, y: i32) -> GlResult {
    unsafe {
        glRasterPos2i(x, y);
    }

    get_error_result()?;

    if !get_boolean(GL_CURRENT_RASTER_POSITION_VALID) {
        Err(GlError::Error(vec![GlSingleError::UnknownError(0)]))
    } else {
        Ok(())
    }
}

pub fn get_string(state_string: StateString) -> Result<String, GlError> {
    let param = state_string.into();

    let byte_ptr = unsafe { glGetString(param) };

    get_error_result()?;

    if byte_ptr.is_null() {
        Err(GlError::UnexpectedError(
            "String pointer is null. GL Context is potentially uninitialized",
        ))
    } else {
        let char_ptr = byte_ptr.cast();
        let cstr = unsafe { CStr::from_ptr(char_ptr) };
        Ok(cstr.to_string_lossy().to_string())
    }
}

pub fn get_boolean(param: GLenum) -> bool {
    let mut boolean_val: GLboolean = 0;

    unsafe {
        glGetBooleanv(param, addr_of_mut!(boolean_val));
    }

    boolean_val == 1
}

pub fn get_floats(state_float: StateFloat) -> Result<Vec<f32>, GlError> {
    let mut ret = vec![0_f32; state_float.return_length()];

    let gl_param: GLenum = state_float.into();
    let raw_ret_pointer = ret.as_mut_ptr();

    unsafe {
        glGetFloatv(gl_param, raw_ret_pointer);
    }

    get_error_result()?;

    Ok(ret)
}

pub fn pixel_store_i(alignment: Alignment, value: AlignmentValue) {
    let pname = alignment.into();
    let param = value.into();

    unsafe {
        glPixelStorei(pname, param);
    }

    debug_assert!(get_error_result().is_ok());
}

pub fn ortho(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near_val: f64,
    far_val: f64,
) -> GlResult {
    unsafe {
        glOrtho(left, right, bottom, top, near_val, far_val);
    }

    get_error_result()
}

pub fn viewport(x: i32, y: i32, width: i32, height: i32) -> GlResult {
    unsafe {
        glViewport(x, y, width, height);
    }

    get_error_result()
}

pub fn disable(capability: Capability) -> GlResult {
    let gl_capability = capability.into();

    unsafe {
        glDisable(gl_capability);
    }

    get_error_result()
}

pub fn enable(capability: Capability) -> GlResult {
    let gl_capability = capability.into();

    unsafe {
        glEnable(gl_capability);
    }

    get_error_result()
}

pub fn matrix_mode(mode: MatrixMode) -> GlResult {
    let gl_mode = mode.into();

    unsafe {
        glMatrixMode(gl_mode);
    }

    get_error_result()
}

pub fn load_identity() -> GlResult {
    unsafe {
        glLoadIdentity();
    }

    get_error_result()
}
