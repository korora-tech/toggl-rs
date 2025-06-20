use super::{ReportFilters, ReportTimeRange};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use toggl_core::{ClientId, ProjectId, WorkspaceId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeeklyReportRequest {
    pub workspace_id: WorkspaceId,
    #[serde(flatten)]
    pub time_range: ReportTimeRange,
    #[serde(flatten)]
    pub filters: Option<ReportFilters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeeklyReportResponse {
    pub week_totals: Vec<WeekTotal>,
    pub projects: Vec<WeeklyProject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeekTotal {
    pub week_start: NaiveDate,
    pub week_end: NaiveDate,
    pub seconds: u64,
    pub billable_seconds: Option<u64>,
    pub amount: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeeklyProject {
    pub project_id: Option<ProjectId>,
    pub project_name: Option<String>,
    pub client_id: Option<ClientId>,
    pub client_name: Option<String>,
    pub color: Option<String>,
    pub billable: bool,
    pub weekly_amounts: Vec<WeeklyAmount>,
    pub total_seconds: u64,
    pub total_billable_seconds: Option<u64>,
    pub total_amount: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeeklyAmount {
    pub week_start: NaiveDate,
    pub seconds: u64,
    pub billable_seconds: Option<u64>,
    pub amount: Option<f64>,
}
