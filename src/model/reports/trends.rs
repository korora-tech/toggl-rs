use super::{ReportFilters, ReportTimeRange};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendsReportRequest {
    pub workspace_id: u64,
    #[serde(flatten)]
    pub time_range: ReportTimeRange,
    #[serde(flatten)]
    pub filters: Option<ReportFilters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectTrendsResponse {
    pub projects: Vec<ProjectTrend>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectTrend {
    pub project_id: Option<u64>,
    pub project_name: Option<String>,
    pub client_id: Option<u64>,
    pub client_name: Option<String>,
    pub color: Option<String>,
    pub data_points: Vec<TrendDataPoint>,
    pub total_seconds: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientTrendsResponse {
    pub clients: Vec<ClientTrend>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientTrend {
    pub client_id: Option<u64>,
    pub client_name: Option<String>,
    pub data_points: Vec<TrendDataPoint>,
    pub total_seconds: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserTrendsResponse {
    pub users: Vec<UserTrend>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserTrend {
    pub user_id: u64,
    pub user_name: String,
    pub data_points: Vec<TrendDataPoint>,
    pub total_seconds: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendDataPoint {
    pub date: NaiveDate,
    pub seconds: u64,
}
