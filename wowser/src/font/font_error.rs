use crate::{from_err, util::StringError};
use std::{
    array::TryFromSliceError,
    convert::Infallible,
    error::Error,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
    str::Utf8Error,
};

#[derive(Debug)]

pub struct FontError {
    err: Box<dyn std::error::Error>,
}

impl From<&str> for FontError {
    fn from(err: &str) -> FontError {
        FontError {
            err: Box::new(StringError::from(err)),
        }
    }
}

impl From<String> for FontError {
    fn from(err: String) -> FontError {
        FontError {
            err: Box::new(StringError::from(err)),
        }
    }
}

from_err!(FontError, StringError);
from_err!(FontError, ParseIntError);
from_err!(FontError, ParseFloatError);
from_err!(FontError, std::io::Error);
from_err!(FontError, Utf8Error);
from_err!(FontError, TryFromSliceError);
from_err!(FontError, TryFromIntError);
from_err!(FontError, Infallible);

impl Error for FontError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.err.as_ref())
    }
}

impl std::fmt::Display for FontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FontError: {}", self.err)
    }
}
