//! Webhook event types and handling

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use toggl_core::{
    ClientId, GroupId, OrganizationId, ProjectId, TagId, TaskId, TimeEntryId, UserId, WorkspaceId,
};

/// Webhook event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    /// Event ID
    pub event_id: String,

    /// When the event was created
    pub created_at: DateTime<Utc>,

    /// Creator user ID (for user-initiated events)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<UserId>,

    /// Event metadata
    pub metadata: EventMetadata,

    /// Event payload (specific to event type)
    pub payload: serde_json::Value,
}

/// Event metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    /// Event action (created, updated, deleted)
    pub action: String,

    /// Event entity type
    pub entity_type: String,

    /// HTTP request ID for tracing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// Event version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_version: Option<String>,
}

/// Common event payload fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPayload<T> {
    /// The entity data
    #[serde(flatten)]
    pub data: T,

    /// Workspace ID where the event occurred
    pub workspace_id: WorkspaceId,

    /// Organization ID (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<OrganizationId>,
}

// Event entity types

/// Time entry event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryEventData {
    pub id: TimeEntryId,
    pub user_id: UserId,
    pub workspace_id: WorkspaceId,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub billable: bool,
    pub start: DateTime<Utc>,
    pub stop: Option<DateTime<Utc>>,
    pub duration: i64,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub tag_ids: Option<Vec<TagId>>,
    pub at: DateTime<Utc>,
}

/// Project event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEventData {
    pub id: ProjectId,
    pub workspace_id: WorkspaceId,
    pub client_id: Option<ClientId>,
    pub name: String,
    pub is_private: bool,
    pub active: bool,
    pub at: DateTime<Utc>,
    pub color: Option<String>,
    pub billable: Option<bool>,
    pub currency: Option<String>,
    pub rate: Option<f64>,
}

/// Client event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientEventData {
    pub id: ClientId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub at: DateTime<Utc>,
}

/// Tag event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagEventData {
    pub id: TagId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub at: DateTime<Utc>,
}

/// Task event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEventData {
    pub id: TaskId,
    pub workspace_id: WorkspaceId,
    pub project_id: ProjectId,
    pub name: String,
    pub active: bool,
    pub at: DateTime<Utc>,
}

/// User event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEventData {
    pub id: UserId,
    pub email: String,
    pub fullname: Option<String>,
    pub at: DateTime<Utc>,
}

/// Workspace user event payload (for workspace membership changes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceUserEventData {
    pub user_id: UserId,
    pub workspace_id: WorkspaceId,
    pub admin: bool,
    pub active: bool,
    pub at: DateTime<Utc>,
}

/// Group event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEventData {
    pub id: GroupId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub at: DateTime<Utc>,
}

/// Webhook event action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventAction {
    Created,
    Updated,
    Deleted,
}

/// Webhook event entity types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventEntityType {
    TimeEntry,
    Project,
    Client,
    Tag,
    Task,
    User,
    WorkspaceUser,
    Group,
}

/// Helper functions for webhook signature verification
#[cfg(feature = "client")]
pub mod verification {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    /// Verify webhook signature
    ///
    /// # Arguments
    /// * `payload` - The raw webhook payload
    /// * `signature` - The signature from the X-Webhook-Signature-256 header (format: "sha256=...")
    /// * `secret` - Your webhook secret
    ///
    /// # Returns
    /// True if the signature is valid
    ///
    /// # Example
    /// ```
    /// let payload = b"webhook payload";
    /// let signature = "sha256=1234abcd..."; // From X-Webhook-Signature-256 header
    /// let secret = "your-webhook-secret";
    ///
    /// if verify_signature(payload, signature, secret) {
    ///     // Process webhook
    /// }
    /// ```
    pub fn verify_signature(payload: &[u8], signature: &str, secret: &str) -> bool {
        // Extract the hash from the "sha256=..." format
        let hash = match signature.strip_prefix("sha256=") {
            Some(h) => h,
            None => {
                // For backward compatibility, try using the signature as-is
                signature
            }
        };

        let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
            Ok(mac) => mac,
            Err(_) => return false,
        };

        mac.update(payload);
        let result = mac.finalize();
        let expected = hex::encode(result.into_bytes());

        // Constant-time comparison
        hash == expected
    }
}

// Re-export for convenience
#[cfg(feature = "client")]
pub use verification::verify_signature;
