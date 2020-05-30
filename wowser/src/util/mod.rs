mod bits;
mod error;
mod string;
mod vec;

pub use bits::{
    get_bit, offset_bit_merge, u4_from_u8, u8_arr_to_u16, u8_to_i32, u8_to_u32, Bit, U4Bit,
};
pub use error::StringError;
pub use string::{byte_to_hex, bytes_to_hex, string_to_bytes, u8_to_str, HexConversion};
pub use vec::{vec_contains, vec_window_split};
