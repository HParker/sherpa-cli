use reqwest::Error as ReqwestError;
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    HttpError(String),
}

use self::Error::*;

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            HttpError(ref description) => description,
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
