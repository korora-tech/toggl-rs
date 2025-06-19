use super::{Pagination, ReportFilters, ReportTimeRange, ReportTotals};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedReportRequest {
    pub workspace_id: u64,
    #[serde(flatten)]
    pub time_range: ReportTimeRange,
    #[serde(flatten)]
    pub filters: Option<ReportFilters>,
    #[serde(flatten)]
    pub pagination: Option<Pagination>,
    pub rounding: Option<i32>,
    pub rounding_minutes: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedReportResponse {
    pub total_count: u32,
    pub per_page: u32,
    pub page: u32,
    pub data: Vec<DetailedReportItem>,
    pub totals: ReportTotals,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedReportItem {
    pub id: u64,
    pub user_id: u64,
    pub user_name: String,
    pub project_id: Option<u64>,
    pub project_name: Option<String>,
    pub project_color: Option<String>,
    pub client_id: Option<u64>,
    pub client_name: Option<String>,
    pub task_id: Option<u64>,
    pub task_name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub billable: bool,
    pub is_billable: bool,
    pub cur: Option<String>,
    pub rate: Option<f64>,
    pub amount: Option<f64>,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub dur: u64,
}
