use std::error::Error as StdError;
use std::io::Error as IoError;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    ParseError(ParseErrorKind),
}

#[derive(Debug)]
pub enum ParseErrorKind {
    UnknownCommand,
    Incomplete,
    InvalidOption,
    NumberFormat,
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::IoError(error)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(_) => "io error",
            Error::ParseError(_) => "parse error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref msg) => write!(fmt, "IO error {}", msg),
            Error::ParseError(ref msg) => write!(fmt, "Parse error {:?}", msg),
        }
    }
}
