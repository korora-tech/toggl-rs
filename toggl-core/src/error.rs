use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// HTTP API errors with status code and message
    #[error("API error ({status}): {message}")]
    ApiError {
        status: u16,
        message: String,
        /// Optional error details from the API response
        details: Option<serde_json::Value>,
    },

    /// Authentication-related errors
    #[error("Authentication error: {0}")]
    AuthError(String),

    /// Rate limiting errors
    #[error("Rate limit exceeded. Retry after {retry_after} seconds")]
    RateLimitError { retry_after: u64 },

    /// Validation errors for request data
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Network-related errors
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Resource not found errors
    #[error("Resource not found: {resource_type} with id {id}")]
    NotFoundError { resource_type: String, id: String },

    /// Permission/access errors
    #[error("Permission denied: {0}")]
    PermissionError(String),

    /// General library errors
    #[error("Library error: {0}")]
    LibraryError(String),

    /// URL parsing errors
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    /// Request building errors
    #[error("Failed to build request: {0}")]
    RequestBuildError(String),

    /// Response parsing errors
    #[error("Failed to parse response: {0}")]
    ResponseParseError(String),
}

impl Error {
    /// Create an API error with optional details
    pub fn api_error(
        status: u16,
        message: impl Into<String>,
        details: Option<serde_json::Value>,
    ) -> Self {
        Self::ApiError {
            status,
            message: message.into(),
            details,
        }
    }

    /// Create a not found error
    pub fn not_found(resource_type: impl Into<String>, id: impl ToString) -> Self {
        Self::NotFoundError {
            resource_type: resource_type.into(),
            id: id.to_string(),
        }
    }

    /// Check if this is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFoundError { .. })
            || matches!(self, Self::ApiError { status: 404, .. })
    }

    /// Check if this is an authentication error
    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::AuthError(_)) || matches!(self, Self::ApiError { status: 401, .. })
    }

    /// Check if this is a rate limit error
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Self::RateLimitError { .. })
            || matches!(self, Self::ApiError { status: 429, .. })
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Extension trait for converting various error types to our Error type
pub trait ErrorExt<T> {
    fn map_err_to_lib(self, context: &str) -> Result<T>;
}

impl<T, E: std::fmt::Display> ErrorExt<T> for std::result::Result<T, E> {
    fn map_err_to_lib(self, context: &str) -> Result<T> {
        self.map_err(|e| Error::LibraryError(format!("{}: {}", context, e)))
    }
}
