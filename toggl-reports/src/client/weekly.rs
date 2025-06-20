//! Weekly report endpoints

use reqwest::Method;
use toggl_core::Result;

use super::ReportsClient;
use crate::models::weekly::*;

pub struct WeeklyClient {
    client: ReportsClient,
}

impl WeeklyClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Get weekly report for a workspace
    pub fn get(&self, workspace_id: i64, request: &WeeklyPost) -> Result<WeeklyReport> {
        let path = format!("/workspace/{}/weekly/time_entries", workspace_id);
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Export weekly report as CSV
    pub fn export_csv(&self, workspace_id: i64, request: &WeeklyExportPost) -> Result<Vec<u8>> {
        let path = format!("/workspace/{}/weekly/time_entries.csv", workspace_id);
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }

    /// Export weekly report as PDF
    pub fn export_pdf(&self, workspace_id: i64, request: &WeeklyExportPDFPost) -> Result<Vec<u8>> {
        let path = format!("/workspace/{}/weekly/time_entries.pdf", workspace_id);
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }
}
