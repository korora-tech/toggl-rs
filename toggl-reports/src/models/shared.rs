//! Shared report models

use serde::{Deserialize, Serialize};

/// Shared report response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedReportResponse {
    pub token: String,
    pub name: String,
    pub type_: String,
    pub data: serde_json::Value,
}

/// Shared report export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedReportExport {
    pub extension: String,
    pub url: String,
}
