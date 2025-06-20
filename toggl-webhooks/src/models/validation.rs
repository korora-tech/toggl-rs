//! Webhook validation models

use serde::{Deserialize, Serialize};

/// Subscription validation DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionValidationDto {
    /// Event type for validation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    /// Validation code to echo back
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_code: Option<String>,
}

/// Subscription validation response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionValidationResponseDto {
    /// Echoed validation code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_code: Option<String>,
}
