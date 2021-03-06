use std::error::Error;
use std::fmt::Display;

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
