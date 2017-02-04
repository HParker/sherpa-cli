use reqwest::Error as ReqwestError;
use std::fmt;
use std::io::Error as IoError;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    HttpError(String),
    ResponseReadError(IoError),
}

use self::Error::*;

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            HttpError(ref description) => description,
            ResponseReadError(ref io_error) => io_error.description(),
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
        HttpError(error.description().into())
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        ResponseReadError(error)
    }
}
