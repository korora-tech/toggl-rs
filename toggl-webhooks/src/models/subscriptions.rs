//! Webhook subscription models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use toggl_core::WorkspaceId;

/// Webhook subscription input DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInDto {
    /// Description of the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the subscription is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Event filters for the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filters: Option<Vec<SubscriptionInEventFilter>>,

    /// Whether the subscription has pending events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pending_events: Option<bool>,

    /// Secret for HMAC signature verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// Callback URL for webhook events
    pub url_callback: String,

    /// User agent header to send with webhook requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    /// Workspace ID for the subscription
    pub workspace_id: WorkspaceId,
}

/// Event filter for subscription input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInEventFilter {
    /// Action type (e.g., "created", "updated", "deleted")
    pub action: String,

    /// Entity type (e.g., "time_entry", "project", "client")
    pub entity: String,
}

/// Webhook subscription output DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionOutDto {
    /// When the subscription was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// Description of the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the subscription is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Event filters for the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filters: Option<Vec<SubscriptionOutEventFilter>>,

    /// Whether the subscription has pending events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pending_events: Option<bool>,

    /// Secret for HMAC signature verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// Subscription ID
    pub subscription_id: String,

    /// When the subscription was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    /// Callback URL for webhook events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_callback: Option<String>,

    /// User agent header to send with webhook requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    /// When the subscription was last validated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated_at: Option<DateTime<Utc>>,

    /// Workspace ID for the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<WorkspaceId>,
}

/// Event filter for subscription output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionOutEventFilter {
    /// Action type (e.g., "created", "updated", "deleted")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Entity type (e.g., "time_entry", "project", "client")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
}

/// Webhook subscription update DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdateDto {
    /// Description of the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the subscription is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Event filters for the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filters: Option<Vec<SubscriptionUpdateEventFilter>>,

    /// Whether the subscription has pending events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pending_events: Option<bool>,

    /// Secret for HMAC signature verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// Callback URL for webhook events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_callback: Option<String>,

    /// User agent header to send with webhook requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

/// Event filter for subscription update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdateEventFilter {
    /// Action type (e.g., "created", "updated", "deleted")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Entity type (e.g., "time_entry", "project", "client")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
}
