use super::{ReportFilters, ReportGrouping, ReportOrdering, ReportTimeRange};
use serde::{Deserialize, Serialize};
use toggl_core::WorkspaceId;

/// Request parameters for generating a summary report.
///
/// Summary reports provide aggregated time tracking data grouped by
/// projects, clients, tags, or users.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
/// use toggl_api::models::reports::{SummaryReportRequest, ReportTimeRange, ReportGrouping};
/// use chrono::Utc;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// let request = SummaryReportRequest {
///     workspace_id,
///     time_range: ReportTimeRange::ThisWeek,
///     grouping: Some(ReportGrouping::Projects),
///     sub_grouping: Some(ReportGrouping::Clients),
///     filters: None,
///     order_by: None,
///     rounding: None,
///     rounding_minutes: None
/// };
///
/// let report = client.reports().summary(request).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryReportRequest {
    /// Workspace to generate the report for
    pub workspace_id: WorkspaceId,
    /// Time period for the report
    #[serde(flatten)]
    pub time_range: ReportTimeRange,
    /// Optional filters to apply
    #[serde(flatten)]
    pub filters: Option<ReportFilters>,
    /// Primary grouping dimension (projects, clients, tags, users)
    #[serde(flatten)]
    pub grouping: Option<ReportGrouping>,
    /// Secondary grouping dimension
    pub sub_grouping: Option<ReportGrouping>,
    /// How to order results
    pub order_by: Option<ReportOrdering>,
    /// Time rounding setting (-1 = round down, 0 = no rounding, 1 = round up)
    pub rounding: Option<i32>,
    /// Minutes to round to (e.g., 5, 10, 15)
    pub rounding_minutes: Option<i32>,
}

/// Response from a summary report request.
///
/// Contains grouped time tracking data and totals.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryReportResponse {
    pub groups: Vec<SummaryGroup>,
    pub total: SummaryTotal,
}

/// A group in the summary report.
///
/// Represents aggregated data for a specific project, client, tag, or user.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryGroup {
    /// ID of the grouped entity (project, client, tag, or user ID)
    pub id: Option<u64>,
    /// Name of the grouped entity
    pub name: Option<String>,
    /// Total duration in seconds
    pub duration: u64,
    /// Billable duration in seconds
    pub billable_duration: Option<u64>,
    /// Total billable amount in workspace currency
    pub amount: Option<f64>,
    /// Number of time entries in this group
    pub count: u32,
    /// Sub-groups if secondary grouping was requested
    pub sub_groups: Option<Vec<SummarySubGroup>>,
}

/// A sub-group within a summary group.
///
/// Represents secondary level aggregation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummarySubGroup {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub duration: u64,
    pub billable_duration: Option<u64>,
    pub amount: Option<f64>,
    pub count: u32,
}

/// Total values across all groups in the summary report.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryTotal {
    pub duration: u64,
    pub billable_duration: Option<u64>,
    pub amount: Option<f64>,
    pub count: u32,
}
