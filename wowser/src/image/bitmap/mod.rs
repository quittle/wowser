#[allow(clippy::module_inception)]
mod bitmap;
mod bitmap_compression_method;
mod bitmap_header;
mod bitmap_info_header;

pub use bitmap::Bitmap;
