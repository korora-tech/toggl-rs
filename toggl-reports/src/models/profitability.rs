//! Profitability report models

use serde::{Deserialize, Serialize};
use toggl_core::{ClientId, GroupId, ProjectId, UserId};

/// Project profitability request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectProfitabilityRequest {
    /// Project IDs
    pub project_ids: Vec<ProjectId>,

    /// Filter parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ProfitabilityFilter>,
}

/// Employee profitability request (insights API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeProfitabilityRequest {
    /// Start date
    pub start_date: String,

    /// End date
    pub end_date: String,

    /// User IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<UserId>>,

    /// Group IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<GroupId>>,

    /// Resolution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

/// Project trends request (insights API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTrendsRequest {
    /// Start date
    pub start_date: String,

    /// End date
    pub end_date: String,

    /// Project IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<ProjectId>>,

    /// Client IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<Vec<ClientId>>,

    /// Billable filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable: Option<bool>,

    /// Resolution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

/// Profitability filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitabilityFilter {
    /// Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    /// End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// Project profitability response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectProfitabilityReport {
    pub data: Vec<ProjectProfitability>,
}

/// Project profitability data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectProfitability {
    /// Project ID
    pub project_id: ProjectId,

    /// Total tracked seconds
    pub total_tracked_seconds: i64,

    /// Total billable seconds
    pub total_billable_seconds: i64,

    /// Billable amount
    pub billable_amount: f64,

    /// Labor cost
    pub labor_cost: f64,

    /// Profit
    pub profit: f64,

    /// Profit margin percentage
    pub profit_margin: f64,

    /// Fixed fee amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_fee: Option<f64>,

    /// Currency
    pub currency: String,
}

/// Employee profitability data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeProfitability {
    /// User ID
    pub user_id: UserId,

    /// User name
    pub user_name: String,

    /// Total tracked seconds
    pub total_tracked_seconds: i64,

    /// Total billable seconds
    pub total_billable_seconds: i64,

    /// Billable percentage
    pub billable_percentage: f64,

    /// Labor cost
    pub labor_cost: f64,

    /// Billable amount
    pub billable_amount: f64,

    /// Profit
    pub profit: f64,

    /// Currency
    pub currency: String,
}

/// Project trends data (insights API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTrends {
    /// Project ID
    pub project_id: ProjectId,

    /// Trend data points
    pub trends: Vec<TrendDataPoint>,
}

/// Trend data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendDataPoint {
    /// Period (date)
    pub period: String,

    /// Total seconds
    pub total_seconds: i64,

    /// Billable seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_seconds: Option<i64>,
}
