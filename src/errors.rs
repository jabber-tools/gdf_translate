use std::result;

#[derive(Debug)]
pub struct Error {
    message: String
}

pub type Result<T> = result::Result<T, Error>;

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Error {
        Error {
            message: format!("{}",error)
        }
    }
}