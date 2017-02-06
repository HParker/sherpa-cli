use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    Http(String),
    Io(IoError),
    JsonError(SerdeError),
}

use self::Error::*;

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Http(ref description) => description,
            Io(ref io_error) => io_error.description(),
            JsonError(ref serde_error) => serde_error.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.description().fmt(f)
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Error {
        Http(error.description().into())
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Io(error)
    }
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Error {
        JsonError(error)
    }
}
