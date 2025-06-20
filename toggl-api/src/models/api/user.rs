use super::ids::{CountryId, TagId, UserId, WorkspaceId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub api_token: Option<String>,
    pub email: String,
    pub fullname: String,
    pub timezone: String,
    pub default_workspace_id: WorkspaceId,
    pub beginning_of_week: u8,
    pub image_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub openid_email: Option<String>,
    pub openid_enabled: bool,
    pub country_id: Option<CountryId>,
    pub at: DateTime<Utc>,
    pub intercom_hash: Option<String>,
    pub has_password: bool,
    pub options: Option<Value>,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub workspace_id: WorkspaceId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUser {
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub timezone: Option<String>,
    pub default_workspace_id: Option<WorkspaceId>,
    pub beginning_of_week: Option<u8>,
    pub country_id: Option<CountryId>,
    pub password: Option<String>,
    pub current_password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserWebTimer {
    pub web_timer_id: String,
    pub web_timer_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LostPassword {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetToken {
    pub token: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupData {
    pub email: String,
    pub password: String,
    pub timezone: Option<String>,
    pub created_with: Option<String>,
    pub terms_agreed: Option<bool>,
    pub terms_agreed_at: Option<DateTime<Utc>>,
    pub marketing_agreed: Option<bool>,
    pub marketing_agreed_at: Option<DateTime<Utc>>,
    pub tos_accepted: Option<bool>,
    pub country_id: Option<CountryId>,
}
