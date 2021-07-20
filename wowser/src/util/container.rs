#[macro_export]
macro_rules! try_option {
    ($a:expr) => {
        match $a {
            Some(v) => v,
            None => {
                return None;
            }
        }
    };
}

#[cfg(test)]
mod tests {
    const CONST_3: Option<u8> = add_one(Some(2));
    const CONST_NONE: Option<u8> = add_one(None);

    /**
     * Verify usable in const expressions
     */
    const fn add_one(value: Option<u8>) -> Option<u8> {
        let value: u8 = try_option!(value);
        Some(value + 1)
    }

    #[test]
    fn test_constants_are_correct() {
        assert_eq!(CONST_3, Some(3));
        assert_eq!(CONST_NONE, None);
    }
}
