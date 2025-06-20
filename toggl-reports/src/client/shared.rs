//! Shared report endpoints

use reqwest::Method;
use toggl_core::Result;

use super::ReportsClient;
use crate::models::shared::*;

pub struct SharedClient {
    client: ReportsClient,
}

impl SharedClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Get shared report by token
    pub fn get(&self, report_token: &str) -> Result<SharedReportResponse> {
        let path = format!("/shared/{}", report_token);
        self.client.request(Method::GET, &path)
    }

    /// Export shared report as CSV
    pub fn export_csv(&self, report_token: &str) -> Result<Vec<u8>> {
        let path = format!("/shared/{}/csv", report_token);
        self.client.request_bytes(Method::GET, &path)
    }

    /// Export shared report as PDF
    pub fn export_pdf(&self, report_token: &str) -> Result<Vec<u8>> {
        let path = format!("/shared/{}/pdf", report_token);
        self.client.request_bytes(Method::GET, &path)
    }

    /// Export shared report as XLSX
    pub fn export_xlsx(&self, report_token: &str) -> Result<Vec<u8>> {
        let path = format!("/shared/{}/xlsx", report_token);
        self.client.request_bytes(Method::GET, &path)
    }
}
