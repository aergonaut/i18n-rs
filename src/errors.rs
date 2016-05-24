use std::result::Result as StdResult;
use std::error;
use std::fmt;
use std::io;
use yaml_rust;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Yaml(yaml_rust::scanner::ScanError),
    Other(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Yaml(ref err) => write!(f, "YAML error: {}", err),
            Error::Other(ref msg) => write!(f, "Other error: {}", msg)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Yaml(ref err) => err.description(),
            Error::Other(ref msg) => msg
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Yaml(ref err) => Some(err),
            Error::Other(_) => None
        }
    }
}

macro_rules! from_error {
    ($($p:ty as $e:ident,)*) => (
        $(impl From<$p> for Error {
            fn from(p: $p) -> Error { Error::$e(p) }
        })*
    )
}

from_error! {
    io::Error as Io,
    yaml_rust::scanner::ScanError as Yaml,
}
