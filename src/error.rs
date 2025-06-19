use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },

    #[error("library error: {0}")]
    LibraryError(String),

    #[cfg(feature = "client")]
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
