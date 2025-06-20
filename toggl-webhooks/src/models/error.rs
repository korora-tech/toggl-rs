//! Error models

use serde::{Deserialize, Serialize};

/// Error response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDto {
    /// Error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,

    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Helpful tip for resolving the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<String>,
}
