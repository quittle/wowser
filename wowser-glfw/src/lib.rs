mod callback;
mod error;
mod glfw;
mod ptr_holder;
mod window_hint;

pub use callback::*;
pub use error::*;
pub use glfw::*;
pub use window_hint::*;

/// Used to convert a callback for usage with this module's set_error_callback
/// ```
/// wowser_glfw::ErrorCallback!(fn my_callback(err: wowser_glfw::GlfwError, m: str) {
///     println!("Err {}: {}", err, m);
/// });
///
/// fn setup() {
///     wowser_glfw::set_error_callback(Some(my_callback));
/// }
/// ```
#[macro_export]
macro_rules! ErrorCallback {
    (fn $fname:ident($arg_err:ident : wowser_glfw::GlfwError, $arg_message:ident : str) $body:block ) => {
        #[no_mangle]
        unsafe extern "C" fn $fname(i: i32, cstr: *const i8) {
            let err = ::wowser_glfw::get_error();
            let s = ::std::ffi::CStr::from_ptr(cstr).to_string_lossy();
            wowser_ErrorCallback::$fname(err, s);
        }

        mod wowser_ErrorCallback {
            pub fn $fname($arg_err: ::wowser_glfw::GlfwError, $arg_message: ::std::borrow::Cow<str>) $body
        }

    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This test fails if no X server is running.
    #[test]
    fn test_init() {
        init().expect("Failure may be due to no X server running");
    }
}
