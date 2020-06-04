use glob::{GlobError, PatternError};
use std::result;
use zip::result::ZipError;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

pub type Result<T> = result::Result<T, Error>;

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<ZipError> for Error {
    fn from(error: ZipError) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<PatternError> for Error {
    fn from(error: PatternError) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<GlobError> for Error {
    fn from(error: GlobError) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}
