use std::{io, str::Utf8Error};

#[derive(Debug)]
pub enum StorageError {
    IoError(io::Error),
    Utf8Error(Utf8Error),
}

impl From<io::Error> for StorageError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<Utf8Error> for StorageError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8Error(error)
    }
}
