use std::error::Error;
use std::fmt::{Display, Formatter};

pub type Result<T> = core::result::Result<T, HttpRequestError>;

/// Represents errors that occur when making an HTTP request
#[derive(Debug)]
pub struct HttpRequestError {
    err: Box<dyn Error>,
}

impl HttpRequestError {
    pub fn from(err: Box<dyn Error>) -> HttpRequestError {
        HttpRequestError { err }
    }
}

impl Error for HttpRequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.err.as_ref())
    }
}

impl Display for HttpRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP Request error: {}", self.err)
    }
}

unsafe impl Send for HttpRequestError {}
