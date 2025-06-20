//! Comparative report models

use super::base::RangePost;
use serde::{Deserialize, Serialize};

/// Comparative report post parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparativePost {
    /// Date range parameters including previous period
    #[serde(flatten)]
    pub range: RangePost,

    /// Project IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,

    /// User IDs filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<i64>>,

    /// Resolution (day, week, month)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,

    /// Whether to include time entry IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_time_entry_ids: Option<bool>,
}

/// Comparative report response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparativeReport {
    pub data: ComparativeData,
}

/// Comparative report data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparativeData {
    /// Comparison graph data
    pub graph: Vec<ReportGraph>,

    /// Total time for current period
    pub total_current: i64,

    /// Total time for previous period
    pub total_previous: i64,

    /// Percentage change
    pub change_percentage: f64,

    /// Resolution used
    pub resolution: String,
}

/// Report graph data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportGraph {
    /// Date or period label
    pub date: String,

    /// Current period value
    pub current: GraphData,

    /// Previous period value
    pub previous: GraphData,
}

/// Graph data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    /// Time in seconds
    pub seconds: i64,

    /// Formatted time string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
}
