//! Summary report endpoints

use reqwest::Method;
use toggl_core::Result;

use super::ReportsClient;
use crate::models::summary::*;

pub struct SummaryClient {
    client: ReportsClient,
}

impl SummaryClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Get summary report for a workspace
    pub fn get(&self, workspace_id: i64, request: &ReportPost) -> Result<Report> {
        let path = format!("/workspace/{}/summary/time_entries", workspace_id);
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Export summary report as CSV
    pub fn export_csv(&self, workspace_id: i64, request: &ExportPost) -> Result<Vec<u8>> {
        let path = format!("/workspace/{}/summary/time_entries.csv", workspace_id);
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }

    /// Export summary report as XLSX
    pub fn export_xlsx(&self, workspace_id: i64, request: &ExportPost) -> Result<Vec<u8>> {
        let path = format!("/workspace/{}/summary/time_entries.xlsx", workspace_id);
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }

    /// Export summary report as PDF
    pub fn export_pdf(&self, workspace_id: i64, request: &ExportPDFPost) -> Result<Vec<u8>> {
        let path = format!("/workspace/{}/summary/time_entries.pdf", workspace_id);
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }
}
