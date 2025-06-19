use super::TogglClient;
use crate::error::Result;
use crate::model::api::exports::{ExportRequest, ExportStatus};
use reqwest::Method;

pub struct ExportsClient {
    client: TogglClient,
}

impl ExportsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Request data export
    pub fn request_export(&self, export_type: &str) -> Result<ExportStatus> {
        let request = ExportRequest {
            export_type: export_type.to_string(),
        };
        self.client
            .request_with_body(Method::POST, "me/export", &request)
    }

    /// Get export status
    pub fn get_status(&self) -> Result<ExportStatus> {
        self.client.request(Method::GET, "me/export")
    }

    /// Download export data
    pub fn download(&self, export_id: &str) -> Result<Vec<u8>> {
        self.client
            .request_binary(Method::GET, &format!("me/export/data/{}.zip", export_id))
    }
}
