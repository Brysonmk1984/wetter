#[derive(Debug)]
pub enum AcceptableError {
    HttpError(reqwest::Error),
    ClientRequestError(u16),
    ServerResponseError(u16),
}

impl From<reqwest::Error> for AcceptableError {
    fn from(error: reqwest::Error) -> AcceptableError {
        AcceptableError::HttpError(error)
    }
}
