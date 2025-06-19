use super::{ReportFilters, ReportTimeRange};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfitabilityReportRequest {
    pub workspace_id: u64,
    #[serde(flatten)]
    pub time_range: ReportTimeRange,
    #[serde(flatten)]
    pub filters: Option<ReportFilters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectProfitabilityResponse {
    pub projects: Vec<ProjectProfitability>,
    pub totals: ProfitabilityTotals,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectProfitability {
    pub project_id: u64,
    pub project_name: String,
    pub client_id: Option<u64>,
    pub client_name: Option<String>,
    pub billable_seconds: u64,
    pub non_billable_seconds: u64,
    pub total_seconds: u64,
    pub billable_amount: f64,
    pub labor_cost: f64,
    pub profit: f64,
    pub profit_margin: f64,
    pub fixed_fee: Option<f64>,
    pub hourly_rate: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmployeeProfitabilityResponse {
    pub employees: Vec<EmployeeProfitability>,
    pub totals: ProfitabilityTotals,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmployeeProfitability {
    pub user_id: u64,
    pub user_name: String,
    pub billable_seconds: u64,
    pub non_billable_seconds: u64,
    pub total_seconds: u64,
    pub billable_amount: f64,
    pub labor_cost: f64,
    pub profit: f64,
    pub profit_margin: f64,
    pub hourly_rate: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfitabilityTotals {
    pub billable_seconds: u64,
    pub non_billable_seconds: u64,
    pub total_seconds: u64,
    pub billable_amount: f64,
    pub labor_cost: f64,
    pub profit: f64,
    pub profit_margin: f64,
}
