use super::{ReportFilters, ReportTimeRange};
use crate::models::api::ids::{ClientId, ProjectId, WorkspaceId};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComparativeReportRequest {
    pub workspace_id: WorkspaceId,
    pub base_period: ReportTimeRange,
    pub comparison_period: ReportTimeRange,
    #[serde(flatten)]
    pub filters: Option<ReportFilters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComparativeReportResponse {
    pub base_period: PeriodData,
    pub comparison_period: PeriodData,
    pub change: ChangeData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeriodData {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_seconds: u64,
    pub billable_seconds: Option<u64>,
    pub total_amount: Option<f64>,
    pub billable_amount: Option<f64>,
    pub projects: Vec<ComparativeProject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComparativeProject {
    pub project_id: Option<ProjectId>,
    pub project_name: Option<String>,
    pub client_id: Option<ClientId>,
    pub client_name: Option<String>,
    pub seconds: u64,
    pub billable_seconds: Option<u64>,
    pub amount: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeData {
    pub total_seconds_change: i64,
    pub total_seconds_change_percentage: f64,
    pub billable_seconds_change: Option<i64>,
    pub billable_seconds_change_percentage: Option<f64>,
    pub total_amount_change: Option<f64>,
    pub total_amount_change_percentage: Option<f64>,
}
