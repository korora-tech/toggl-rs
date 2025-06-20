//! Filter models for reports API

use serde::{Deserialize, Serialize};

/// Client filter parameters request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientFilterParamsRequest {
    /// Name filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Client filter response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientFilterResponse {
    pub id: i64,
    pub name: String,
    pub archived: bool,
}

/// Project filter parameters request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFilterParamRequest {
    /// Name filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Client IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<Vec<i64>>,

    /// Active status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,

    /// Billable status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_billable: Option<bool>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// Project filter response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFilterResponse {
    pub id: i64,
    pub name: String,
    pub client_id: Option<i64>,
    pub active: bool,
    pub billable: bool,
    pub color: String,
}

/// User filter parameters request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFilterParamsRequest {
    /// Name filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Active status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// User filter response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFilterResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub active: bool,
}

/// Project status parameters request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatusParamsRequest {
    /// Project IDs
    pub project_ids: Vec<i64>,
}

/// Project status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatusResponse {
    pub id: i64,
    pub active: bool,
    pub billable: bool,
}

/// Task status parameters request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusParamsRequest {
    /// Task IDs
    pub task_ids: Vec<i64>,
}

/// Task status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusResponse {
    pub id: i64,
    pub active: bool,
}

/// Project group parameters request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGroupParamsRequest {
    /// Name filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Project IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
}

/// Project group response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGroupResponse {
    pub id: i64,
    pub name: String,
    pub project_ids: Vec<i64>,
}

/// Project user parameters request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUserParamsRequest {
    /// Name filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Project IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
}

/// Project user response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
}

/// Project users request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUsersRequest {
    /// Project IDs
    pub project_ids: Vec<i64>,
}

/// Task request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasksRequest {
    /// Active status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Task IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,

    /// Name filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,

    /// Project active status filter  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_active: Option<bool>,

    /// Project IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,

    /// Start index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,

    /// User IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<i64>>,
}

/// Task response model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Task ID
    pub id: i64,

    /// Task name
    pub name: String,

    /// Task active status
    pub active: bool,

    /// Project ID
    pub project_id: i64,

    /// Project name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,

    /// Client name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,

    /// Estimated seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_seconds: Option<i64>,

    /// When created/modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,

    /// Project color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_color: Option<String>,

    /// Is project billable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_billable: Option<bool>,

    /// Rate for this task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,

    /// Rate last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_last_updated: Option<String>,
}
