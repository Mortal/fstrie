use std::{result, fmt};
use std::os::raw::c_uint;

#[derive(Debug)]
pub enum ErrorKind {
    InternalError,
    OddError,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

pub type Result<T> = result::Result<T, Error>;

impl Into<Error> for ErrorKind {
    fn into(self) -> Error {
        Error { kind: self }
    }
}

impl Error {
    pub fn get_error_code(&self) -> c_uint {
        match self.kind {
            ErrorKind::InternalError => 1,
            ErrorKind::OddError => 2,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Defer to Debug
        write!(f, "{:?}", self)
    }
}
