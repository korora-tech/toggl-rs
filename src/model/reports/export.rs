use super::{ReportFilters, ReportTimeRange};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportRequest {
    pub workspace_id: u64,
    #[serde(flatten)]
    pub time_range: ReportTimeRange,
    #[serde(flatten)]
    pub filters: Option<ReportFilters>,
    pub format: ExportFormat,
    pub rounding: Option<i32>,
    pub rounding_minutes: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Xlsx,
    Pdf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportResponse {
    pub download_url: String,
    pub export_id: String,
    pub status: ExportStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ExportStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportStatusResponse {
    pub export_id: String,
    pub status: ExportStatus,
    pub download_url: Option<String>,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}
