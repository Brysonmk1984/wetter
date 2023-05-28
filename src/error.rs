use std::num::ParseIntError;

#[derive(Debug)]
pub enum CombinedError {
    HttpError(reqwest::Error),
    ClientRequestError(u16),
    ServerResponseError(u16),
    ParseError(ParseIntError),
    InvalidArgument(String),
    TooManyArguments(String),
}

impl From<reqwest::Error> for CombinedError {
    fn from(error: reqwest::Error) -> CombinedError {
        CombinedError::HttpError(error)
    }
}
