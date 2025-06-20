//! Detailed report models

use super::base::{OrderDirection, Post};
use super::dictionary::ReportDictionaries;
use serde::{Deserialize, Serialize};
use toggl_core::{ClientId, ProjectId, TagId, TaskId, TimeEntryId, UserId};

/// Detailed report post parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedPost {
    /// Base filtering parameters
    #[serde(flatten)]
    pub base: Post,

    /// Order by field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,

    /// Order direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_dir: Option<OrderDirection>,

    /// Page number (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,

    /// Grouping option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<String>,

    /// Whether to include time entry IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_time_entry_ids: Option<bool>,

    /// Whether to hide amounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_amounts: Option<bool>,
}

/// Detailed report response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedReport {
    pub data: DetailedReportData,
    pub dictionaries: Option<ReportDictionaries>,
}

/// Detailed report data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DetailedReportData {
    /// Ungrouped time entries
    Single(Vec<SingleTimeEntry>),
    /// Grouped time entries
    Grouped(Vec<GroupedTimeEntry>),
}

/// Single time entry in detailed report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleTimeEntry {
    pub id: TimeEntryId,
    pub user_id: UserId,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub client_id: Option<ClientId>,
    pub description: Option<String>,
    pub billable: bool,
    pub start: String,
    pub stop: Option<String>,
    pub duration: i64,
    pub tag_ids: Option<Vec<TagId>>,

    /// Amount (if billable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Grouped time entries in detailed report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupedTimeEntry {
    /// Group identifier (e.g., date, project ID, etc.)
    pub id: Option<String>,

    /// Group title
    pub title: Option<String>,

    /// Time entries in this group
    pub time_entries: Vec<SingleTimeEntry>,

    /// Group total time
    pub total_time: i64,

    /// Group total amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
}

/// Totals for detailed report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedTotals {
    pub time: i64,
    pub amount: Option<f64>,
    pub count: i32,
}

/// Export parameters for detailed search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchExportPost {
    /// Base detailed parameters
    #[serde(flatten)]
    pub detailed: DetailedPost,

    /// Export format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
}

/// PDF export parameters for detailed report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPDFPost {
    /// Base detailed parameters
    #[serde(flatten)]
    pub detailed: DetailedPost,

    /// Display mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<String>,

    /// Date format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,

    /// Duration format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_format: Option<String>,
}
