//! Definition of the Error type for the crate.

/// The error type for this crate.
/// It is used to harmonize the error types of the dependencies and to add some custom errors.
#[derive(Debug)]
pub enum Error {
    /// An error that occurred while serializing or deserializing JSON.
    SerdeJson(serde_json::Error),
    /// An error that occurred while making a request.
    Reqwest(reqwest::Error),
    InvalidSort,
    InvalidFilter,
    InvalidSecondaryItemType,
    Other(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SerdeJson(error) => write!(formatter, "SerdeJson error: {}", error),
            Self::Reqwest(error) => write!(formatter, "Reqwest error: {}", error),
            Self::InvalidSort => write!(formatter, "Invalid sort"),
            Self::InvalidFilter => write!(formatter, "Invalid filter"),
            Self::InvalidSecondaryItemType => write!(formatter, "Invalid secondary item type"),
            Self::Other(message) => write!(formatter, "{}", message),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJson(error)
    }
}
