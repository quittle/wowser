extern crate wowser_glfw;

/// See util::logging::log for more details
macro_rules! log {
    ($($args:tt)+) => (
        $crate::log!($($args)+)
    );
}

pub mod browser;
pub mod css;
pub mod font;
pub mod html;
pub mod image;
pub mod math_parse;
pub mod net;
pub mod parse;
pub mod render;
pub mod startup;
pub mod ui;
pub mod util;
