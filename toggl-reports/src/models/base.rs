//! Base models for reports API

use serde::{Deserialize, Serialize};
use toggl_core::{ClientId, ProjectId, TagId, TaskId, UserId};

/// Common post parameters for reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    /// Start date in YYYY-MM-DD format
    pub start_date: String,

    /// End date in YYYY-MM-DD format
    pub end_date: String,

    /// Client IDs filter, can include null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<Vec<Option<ClientId>>>,

    /// Project IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<ProjectId>>,

    /// User IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<UserId>>,

    /// Tag IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<TagId>>,

    /// Task IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_ids: Option<Vec<TaskId>>,

    /// Filter by billable status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable: Option<bool>,

    /// Filter by description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Minimum duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration_seconds: Option<i64>,

    /// Maximum duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_seconds: Option<i64>,

    /// Whether to round time entries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rounding: Option<bool>,

    /// Rounding minutes (allowed values: 0, 1, 5, 6, 10, 12, 15, 30, 60, 240)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rounding_minutes: Option<i32>,
}

/// Range post parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangePost {
    /// Start date
    pub start_date: String,

    /// End date
    pub end_date: String,

    /// Previous start date for comparison
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_start_date: Option<String>,

    /// Previous end date for comparison
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_end_date: Option<String>,
}

/// Data trends post parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTrendsPost {
    /// Start date
    pub start_date: String,

    /// End date
    pub end_date: String,

    /// Filter by project IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<ProjectId>>,

    /// Filter by client IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<Vec<Option<ClientId>>>,

    /// Filter by user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<UserId>>,

    /// Filter by billable status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable: Option<bool>,
}

/// Billable hourly rate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillableHourlyRate {
    /// Currency code
    pub currency: String,

    /// Hourly rate amount
    pub amount: f64,
}

/// Common grouping options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupingOption {
    Projects,
    Clients,
    Users,
}

/// Sub-grouping options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubGroupingOption {
    Projects,
    Clients,
    Users,
    TimeEntries,
}

/// Order direction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderDirection {
    Asc,
    Desc,
}

/// Common error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub code: Option<i32>,
}
