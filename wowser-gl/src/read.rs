use std::ffi::c_void;

use crate::{get_error_result, Format, GlError, PixelDataType};

pub fn read_pixels(
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    format: Format,
    pixel_data_type: PixelDataType,
) -> Result<Vec<u8>, GlError> {
    // TODO: Support more types
    assert_eq!(pixel_data_type, PixelDataType::UnsignedByte);

    let mut pixels = vec![0_u8; width * height * format.get_stride() as usize];

    let width = width as i32;
    let height = height as i32;
    let format = format.into();
    let pixel_data_type = pixel_data_type.into();
    let pixels_ptr = pixels.as_mut_ptr() as *mut c_void;
    unsafe {
        wowser_gl_sys::glReadPixels(x, y, width, height, format, pixel_data_type, pixels_ptr);
    }

    get_error_result()?;

    Ok(pixels)
}
