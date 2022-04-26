pub trait NumberUtils {
    /// Performs division, rounding up instead of down as per standard int division.
    ///
    /// This is a temporary implementation.
    /// See <https://github.com/rust-lang/rust/issues/88581> for the official replacement.
    fn div_ceiling(&self, other: Self) -> Self;
}

macro_rules! NumberUtilsImpl {
    ($type: ty) => {
        impl NumberUtils for $type {
            fn div_ceiling(&self, other: Self) -> Self {
                let div = self / other;
                if self % other == 0 {
                    div
                } else {
                    div + 1
                }
            }
        }
    };
}

NumberUtilsImpl!(usize);
NumberUtilsImpl!(u128);
NumberUtilsImpl!(u64);
NumberUtilsImpl!(u32);
NumberUtilsImpl!(u16);
NumberUtilsImpl!(u8);

#[cfg(test)]
mod tests {
    use crate::util::NumberUtils;

    #[test]
    fn test_div_ceil() {
        assert_eq!(8_u8.div_ceiling(4), 2);
        assert_eq!(8_u8.div_ceiling(5), 2);
        assert_eq!(8_u8.div_ceiling(3), 3);
    }
}
