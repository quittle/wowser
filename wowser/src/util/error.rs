use std::error::Error;
use std::fmt::Display;

/// Helper macro for creating From implementations for errors
/// ```
/// # #[macro_use] extern crate wowser;
///
/// struct MyError {
///     err: Box<dyn std::error::Error>
/// }
///
/// from_err!(MyError, std::str::Utf8Error);
///
/// fn convert(bytes: &[u8]) -> Result<String, MyError> {
///     let stringified = std::str::from_utf8(bytes)?;
///     Ok(String::from(stringified))
/// }
/// ```
#[macro_export]
macro_rules! from_err {
    ( $to_type:ty, $from_type:ty  ) => {
        impl From<$from_type> for $to_type {
            fn from(err: $from_type) -> Self {
                Self { err: Box::new(err) }
            }
        }
    };
}

/// Easily converts a String to an error
#[derive(Debug)]
pub struct StringError {
    error: String,
}

impl Error for StringError {}

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.error)
    }
}

impl From<String> for StringError {
    fn from(error: String) -> Self {
        StringError { error }
    }
}

impl From<&str> for StringError {
    fn from(error: &str) -> Self {
        StringError {
            error: error.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_error() {
        let err: Box<dyn Error> = Box::new(StringError::from("abc"));
        assert_eq!("Error: abc", err.to_string());
    }
}
