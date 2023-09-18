use std::io;

macro_rules! make_err {
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

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(format!("io error {}", value.raw_os_error().unwrap()))
    }
}

pub mod util;
pub mod exec;
pub mod pkg;
pub mod task;
pub mod cli;
