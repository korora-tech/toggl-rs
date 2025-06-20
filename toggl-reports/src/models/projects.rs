//! Project-specific report models

use serde::{Deserialize, Serialize};
use toggl_core::{ClientId, ProjectId, TimeEntryId};

/// Project summary request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummaryRequest {
    /// Start date
    pub start_date: String,

    /// End date
    pub end_date: String,

    /// Whether to include time entry IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_time_entry_ids: Option<bool>,
}

/// Project summary response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummaryResponse {
    pub project_id: ProjectId,
    pub tracked_seconds: i64,
    pub billable_seconds: i64,
    pub billable_amount: Option<f64>,
    pub labor_cost: Option<f64>,
    pub currency: Option<String>,

    /// Time entry IDs if requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_entry_ids: Option<Vec<TimeEntryId>>,
}

/// Projects summary response (multiple projects)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectsSummaryResponse {
    pub data: Vec<ProjectSummaryData>,
}

/// Project summary data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummaryData {
    pub project_id: ProjectId,
    pub project_name: String,
    pub client_id: Option<ClientId>,
    pub client_name: Option<String>,
    pub tracked_seconds: i64,
    pub billable_seconds: i64,
    pub billable_amount: Option<f64>,
    pub labor_cost: Option<f64>,
    pub currency: Option<String>,
}
