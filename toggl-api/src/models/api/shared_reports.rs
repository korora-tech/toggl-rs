use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedReport {
    pub deleted_at: Option<String>,
    pub fixed_daterange: Option<bool>,
    pub id: Option<i64>,
    #[serde(rename = "isNAResource")]
    pub is_na_resource: Option<bool>,
    pub is_commenting_enabled: Option<bool>,
    pub name: Option<String>,
    pub params: Option<String>,
    pub public: Option<bool>,
    pub scheduled_email_gids: Option<Vec<i64>>,
    pub scheduled_email_uids: Option<Vec<i64>>,
    pub token: Option<String>,
    pub uid: Option<i64>,
    pub updated_at: Option<String>,
    pub updated_by: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSavedReportPayload {
    pub fixed_daterange: Option<bool>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regenerate_token: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSavedReportPayload {
    pub id: i64,
    pub fixed_daterange: Option<bool>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regenerate_token: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SharedReportsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_dates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestingUserID")]
    pub requesting_user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkDeleteRequest {
    pub ids: Vec<i64>,
}
