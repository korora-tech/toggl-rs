use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportRequest {
    pub export_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportStatus {
    pub status: String,
    pub export_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadRequestRecord {
    pub token: Option<String>,
    pub state: Option<String>,
    pub error_message: Option<String>,
}
