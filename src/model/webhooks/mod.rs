use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookSubscription {
    pub subscription_id: u64,
    pub workspace_id: u64,
    pub user_id: u64,
    pub description: String,
    pub url_callback: String,
    pub event_filters: Vec<WebhookEventFilter>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub validated_at: Option<DateTime<Utc>>,
    pub has_pending_events: bool,
    pub secret: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookEventFilter {
    pub entity: WebhookEntity,
    pub action: WebhookAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEntity {
    TimeEntry,
    Project,
    Client,
    Tag,
    Task,
    User,
    Workspace,
    WorkspaceUser,
    Group,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WebhookAction {
    Created,
    Updated,
    Deleted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateWebhookSubscription {
    pub workspace_id: u64,
    pub description: String,
    pub url_callback: String,
    pub event_filters: Vec<WebhookEventFilter>,
    pub enabled: Option<bool>,
    pub secret: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateWebhookSubscription {
    pub description: Option<String>,
    pub url_callback: Option<String>,
    pub event_filters: Option<Vec<WebhookEventFilter>>,
    pub enabled: Option<bool>,
    pub secret: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookEvent {
    pub event_id: String,
    pub event_type: String,
    pub created_at: DateTime<Utc>,
    pub creator_id: u64,
    pub metadata: WebhookMetadata,
    pub payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookMetadata {
    pub request_type: String,
    pub path: String,
    pub event_user_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookPingResponse {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookLimits {
    pub max_subscriptions_per_workspace: u32,
    pub max_event_filters_per_subscription: u32,
    pub max_pending_events: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookValidation {
    pub subscription_id: u64,
    pub validation_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookState {
    pub enabled: bool,
}
