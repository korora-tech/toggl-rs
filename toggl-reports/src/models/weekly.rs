//! Weekly report models

use super::base::Post;
use serde::{Deserialize, Serialize};

/// Weekly report post parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyPost {
    /// Base filtering parameters
    #[serde(flatten)]
    pub base: Post,

    /// Grouping option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<String>,

    /// Whether to calculate totals
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculate: Option<String>,
}

/// Weekly report response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyReport {
    pub data: Vec<WeeklyData>,
    pub week_totals: Vec<WeekTotal>,
}

/// Weekly data entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyData {
    /// User ID
    pub user_id: i64,

    /// User details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserDetails>,

    /// Weekly totals by project/client
    pub details: Vec<WeeklyDetail>,

    /// Total seconds for the user
    pub total_seconds: i64,

    /// Total billable seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_billable_seconds: Option<i64>,

    /// Total currency amounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_currencies: Option<Vec<CurrencyAmount>>,
}

/// User details in weekly report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetails {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

/// Weekly detail entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyDetail {
    /// Project or client ID (depending on grouping)
    pub id: Option<i64>,

    /// Title/name
    pub title: Option<String>,

    /// Time entries by week
    pub time_entries: Vec<WeekEntry>,

    /// Total seconds
    pub total_seconds: i64,
}

/// Week entry with time data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekEntry {
    /// Week start date (Monday)
    pub week_start: String,

    /// Seconds worked
    pub seconds: i64,
}

/// Week total across all users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekTotal {
    /// Week start date
    pub week_start: String,

    /// Total seconds
    pub seconds: i64,
}

/// Currency amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyAmount {
    pub currency: String,
    pub amount: f64,
}

/// Export parameters for weekly report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyExportPost {
    /// Base weekly parameters
    #[serde(flatten)]
    pub weekly: WeeklyPost,
}

/// PDF export parameters for weekly report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyExportPDFPost {
    /// Base weekly parameters
    #[serde(flatten)]
    pub weekly: WeeklyPost,

    /// Display mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<String>,
}
