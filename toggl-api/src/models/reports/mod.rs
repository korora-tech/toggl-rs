pub mod comparative;
pub mod detailed;
pub mod export;
pub mod profitability;
pub mod summary;
pub mod trends;
pub mod weekly;

pub use comparative::*;
pub use detailed::*;
pub use export::*;
pub use profitability::*;
pub use summary::*;
pub use trends::*;
pub use weekly::*;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use toggl_core::{ClientId, ProjectId, TagId, TaskId, UserId};

/// Time range specification for reports.
///
/// Defines the start and end dates for report data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportTimeRange {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

/// Filters available for report queries.
///
/// All filters are optional and can be combined to narrow down report results.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReportFilters {
    /// Filter by specific clients
    pub client_ids: Option<Vec<ClientId>>,
    /// Filter by specific projects
    pub project_ids: Option<Vec<ProjectId>>,
    /// Filter by specific users
    pub user_ids: Option<Vec<UserId>>,
    /// Filter by specific tags
    pub tag_ids: Option<Vec<TagId>>,
    /// Filter by specific tasks
    pub task_ids: Option<Vec<TaskId>>,
    /// Filter by billable status
    pub billable: Option<bool>,
    /// Filter by description text (partial match)
    pub description: Option<String>,
}

/// Grouping options for report data.
///
/// Controls how report data is aggregated and organized.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReportGrouping {
    pub projects: Option<bool>,
    pub clients: Option<bool>,
    pub users: Option<bool>,
    pub tags: Option<bool>,
    pub tasks: Option<bool>,
}

/// Ordering specification for report results.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportOrdering {
    pub field: String,
    pub direction: OrderDirection,
}

/// Sort direction for report ordering.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OrderDirection {
    Asc,
    Desc,
}

/// Pagination parameters for report results.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// Total values commonly found in report responses.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportTotals {
    /// Total time in seconds
    pub time: u64,
    /// Total billable time in seconds
    pub billable_time: Option<u64>,
    /// Total amount in workspace currency
    pub amount: Option<f64>,
    /// Total billable amount in workspace currency
    pub billable_amount: Option<f64>,
    /// Number of entries
    pub count: u32,
}
