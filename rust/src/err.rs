use std::{result, fmt, str, io};
use std::os::raw::c_uint;

#[derive(Debug)]
pub enum ErrorKind {
    Internal,
    UnicodeDecode(str::Utf8Error),
    RootDoesNotExist,
    Io(io::Error),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

pub type Result<T> = result::Result<T, Error>;

impl From<str::Utf8Error> for Error {
    fn from(e: str::Utf8Error) -> Error {
        ErrorKind::UnicodeDecode(e).into()
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        ErrorKind::Io(e).into()
    }
}

impl Into<Error> for ErrorKind {
    fn into(self) -> Error {
        Error { kind: self }
    }
}

impl Error {
    pub fn get_error_code(&self) -> c_uint {
        match self.kind {
            ErrorKind::Internal => 1,
            ErrorKind::UnicodeDecode(_) => 2,
            ErrorKind::RootDoesNotExist => 3,
            ErrorKind::Io(_) => 4,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Defer to Debug
        write!(f, "{:?}", self)
    }
}
