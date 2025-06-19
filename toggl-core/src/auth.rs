//! Authentication types and utilities

/// Authentication credentials for Toggl API
#[derive(Debug, Clone)]
pub enum Auth {
    /// API token authentication
    ApiToken(String),
    /// Basic authentication with email and password
    Basic { email: String, password: String },
}

impl Auth {
    /// Create new API token authentication
    pub fn api_token(token: impl Into<String>) -> Self {
        Self::ApiToken(token.into())
    }

    /// Create new basic authentication
    pub fn basic(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self::Basic {
            email: email.into(),
            password: password.into(),
        }
    }

    /// Get authorization header value
    pub fn to_header_value(&self) -> String {
        match self {
            Auth::ApiToken(token) => {
                let credentials = base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    format!("{}:api_token", token),
                );
                format!("Basic {}", credentials)
            }
            Auth::Basic { email, password } => {
                let credentials = base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    format!("{}:{}", email, password),
                );
                format!("Basic {}", credentials)
            }
        }
    }
}
