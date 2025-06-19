use super::{ReportFilters, ReportGrouping, ReportOrdering, ReportTimeRange};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryReportRequest {
    pub workspace_id: u64,
    #[serde(flatten)]
    pub time_range: ReportTimeRange,
    #[serde(flatten)]
    pub filters: Option<ReportFilters>,
    #[serde(flatten)]
    pub grouping: Option<ReportGrouping>,
    pub sub_grouping: Option<ReportGrouping>,
    pub order_by: Option<ReportOrdering>,
    pub rounding: Option<i32>,
    pub rounding_minutes: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryReportResponse {
    pub groups: Vec<SummaryGroup>,
    pub total: SummaryTotal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryGroup {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub duration: u64,
    pub billable_duration: Option<u64>,
    pub amount: Option<f64>,
    pub count: u32,
    pub sub_groups: Option<Vec<SummarySubGroup>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummarySubGroup {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub duration: u64,
    pub billable_duration: Option<u64>,
    pub amount: Option<f64>,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryTotal {
    pub duration: u64,
    pub billable_duration: Option<u64>,
    pub amount: Option<f64>,
    pub count: u32,
}
