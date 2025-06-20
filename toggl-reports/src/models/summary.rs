//! Summary report models

use super::base::{GroupingOption, OrderDirection, Post, SubGroupingOption};
use super::dictionary::ReportDictionaries;
use serde::{Deserialize, Serialize};

/// Summary report post parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportPost {
    /// Base filtering parameters
    #[serde(flatten)]
    pub base: Post,

    /// Grouping option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<GroupingOption>,

    /// Sub-grouping option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_grouping: Option<SubGroupingOption>,

    /// Order by field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,

    /// Order direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_dir: Option<OrderDirection>,

    /// Whether to include time entry IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_time_entry_ids: Option<bool>,

    /// Whether to distinguish rates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinguish_rates: Option<bool>,

    /// Resolution for time-based grouping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,

    /// Whether to hide amounts (for non-billable users)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_amounts: Option<bool>,
}

/// Summary report response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub data: Vec<ReportData>,
    pub totals: ReportTotals,
    pub dictionaries: Option<ReportDictionaries>,
}

/// Summary report data entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    /// Group ID (project, client, or user ID depending on grouping)
    pub id: Option<i64>,

    /// Time in seconds
    pub time: i64,

    /// Title/name of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Sub-groups if sub-grouping is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_groups: Option<Vec<SubGroupData>>,

    /// Project IDs if grouped by something else
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,

    /// Amount (if billable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Sub-group data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGroupData {
    /// Sub-group ID
    pub id: Option<i64>,

    /// Title/name
    pub title: Option<String>,

    /// Time in seconds
    pub time: i64,

    /// Amount (if billable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

/// Report totals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTotals {
    /// Total time in seconds
    pub time: i64,

    /// Total amount (if billable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Export parameters for summary reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPost {
    /// Base report parameters
    #[serde(flatten)]
    pub report: ReportPost,

    /// Export format hint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
}

/// PDF export parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPDFPost {
    /// Base report parameters
    #[serde(flatten)]
    pub report: ReportPost,

    /// Display mode for PDF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<String>,
}
