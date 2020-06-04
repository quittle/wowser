use crate::util::StringError;
use std::{
    error::Error,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug)]
pub enum FontError {
    Error(StringError),
    IOError(std::io::Error),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
}

impl From<StringError> for FontError {
    fn from(err: StringError) -> FontError {
        FontError::Error(err)
    }
}

impl From<ParseIntError> for FontError {
    fn from(err: ParseIntError) -> FontError {
        FontError::ParseIntError(err)
    }
}

impl From<ParseFloatError> for FontError {
    fn from(err: ParseFloatError) -> FontError {
        FontError::ParseFloatError(err)
    }
}

impl From<&str> for FontError {
    fn from(err: &str) -> FontError {
        FontError::Error(StringError::from(err))
    }
}

impl From<String> for FontError {
    fn from(err: String) -> FontError {
        FontError::Error(StringError::from(err))
    }
}

impl From<std::io::Error> for FontError {
    fn from(err: std::io::Error) -> FontError {
        FontError::IOError(err)
    }
}

impl Error for FontError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FontError::Error(err) => Some(err),
            FontError::IOError(err) => Some(err),
            FontError::ParseIntError(err) => Some(err),
            FontError::ParseFloatError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for FontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontError::Error(err) => write!(f, "FontError: {}", err),
            FontError::IOError(err) => write!(f, "FontError: {}", err),
            FontError::ParseIntError(err) => write!(f, "FontError: {}", err),
            FontError::ParseFloatError(err) => write!(f, "FontError: {}", err),
        }
    }
}
