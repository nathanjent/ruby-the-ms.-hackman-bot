use std::error::Error as StdError;
use std::io::Error as IoError;
use std::fmt;
use std::any::Any;

pub type Result<T> = ::std::result::Result<T, Error>;
type BoxAny = Box<Any + Send>;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    ParseError(ParseErrorKind),
    UnintentionalBreak,
    UnknownCommand,
    UnknownError(BoxAny),
}

#[derive(Debug)]
pub enum ParseErrorKind {
    UnknownCommand,
    Eof,
    Incomplete,
    InvalidOption,
    NumberFormat,
}

impl From<BoxAny> for Error {
    fn from(error: BoxAny) -> Self {
        Error::UnknownError(error)
    }
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
            Error::UnknownError(_) => "unknown error",
            Error::UnintentionalBreak => "application ended unexpectedly",
            Error::UnknownCommand => "Command is not supported",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            _ => None,
        }
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseErrorKind::UnknownCommand => write!(fmt, "Unknown command"),
            ParseErrorKind::Eof => write!(fmt, "End of file"),
            ParseErrorKind::Incomplete => write!(fmt, "Incomplete command"),
            ParseErrorKind::InvalidOption => write!(fmt, "Invalid command option"),
            ParseErrorKind::NumberFormat => write!(fmt, "Could not parse as a number"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref msg) => write!(fmt, "IO error {}", msg),
            Error::ParseError(ref msg) => write!(fmt, "Parse error {}", msg),
            Error::UnknownError(ref msg) => write!(fmt, "Unknown error {:?}", msg),
            Error::UnintentionalBreak => write!(fmt, "Application ended unexpectedly"),
            Error::UnknownCommand => write!(fmt, "Command is not supported"),
        }
    }
}
