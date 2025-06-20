//! Health check models

use serde::{Deserialize, Serialize};

/// Application health output DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppHealthOutDto {
    /// Health check description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Error message if unhealthy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Health check message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,

    /// Health status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
