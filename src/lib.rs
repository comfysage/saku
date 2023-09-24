use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! make_err {
    () => {
        Error::default()
    };
    ($t:ident, $v:literal) => {
        Error::$t(format!($v))
    };
}

#[derive(Debug, Default)]
pub enum Error {
  NotFound(String),
  Missing(String),
  // io object already exists, multiple packages with similar names
  Conflict(String),
  IO(String),
  #[default]
  Unexpected,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::NotFound(s) => format!("Not Found: {s}"),
            Error::Missing(s) => format!("Missing: {s}"),
            Error::Conflict(s) => format!("Conflict: {s}"),
            Error::IO(s) => format!("IO: {s}"),
            Error::Unexpected => format!("unexpected error"),
        };
        write!(f, "{}", msg)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(format!("io error {}", value.raw_os_error().unwrap()))
    }
}

pub mod util;
pub mod exec;
pub mod pkg;
pub mod cli;
