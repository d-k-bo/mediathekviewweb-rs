use std::fmt::Display;

use crate::models::ApiError;

/// Alias for the `Result`s returned by this library.
pub type Result<T> = core::result::Result<T, Error>;

/// An error returned by this client.
#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    EmptyResponse,
    Response(ApiError),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(_) => f.write_str("HTTP request failed"),
            Error::EmptyResponse => {
                f.write_str("mediathekviewweb server returned an empty response")
            }
            Error::Response(_) => f.write_str("mediathekviewweb server returned an error"),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Reqwest(e) => Some(e),
            Error::EmptyResponse => None,
            Error::Response(e) => Some(e),
        }
    }
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}
impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Error::Response(e)
    }
}
