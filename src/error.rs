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
    UnknownError(BoxAny),
    UnintentionalBreak,
    PlayerNotFound(String),
}

#[derive(Debug)]
pub enum ParseErrorKind {
    UnknownCommand,
    Incomplete,
    InvalidCellType,
    NumberFormat(BoxAny),
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
            Error::UnintentionalBreak => "unintentional break error",
            Error::PlayerNotFound(_) => "player not found error",
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
            ParseErrorKind::Incomplete => write!(fmt, "Incomplete command"),
            ParseErrorKind::InvalidCellType => write!(fmt, "Invalid field cell type"),
            ParseErrorKind::NumberFormat(ref e) => write!(fmt, "Could not parse value {:?}", e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref msg) => write!(fmt, "IO error {}", msg),
            Error::ParseError(ref msg) => write!(fmt, "Parse error {}", msg),
            Error::UnknownError(ref msg) => write!(fmt, "Unknown error {:?}", msg),
            Error::UnintentionalBreak => write!(fmt, "Unintentional break error"),
            Error::PlayerNotFound(ref name) => write!(fmt, "Player not found error {}", name),
        }
    }
}
