use super::TogglClient;
use crate::{
    error::Result,
    model::api::{CreateScheduledReportPayload, ScheduledReport},
};
use reqwest::Method;

pub struct ScheduledReportsClient {
    client: TogglClient,
}

impl ScheduledReportsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// List scheduled reports
    pub fn list_scheduled_reports(&self, workspace_id: u64) -> Result<Vec<ScheduledReport>> {
        let url = format!("/workspaces/{}/scheduled_reports", workspace_id);
        self.client.request(Method::GET, &url)
    }

    /// Create a scheduled report
    pub fn create_scheduled_report(
        &self,
        workspace_id: u64,
        report: CreateScheduledReportPayload,
    ) -> Result<ScheduledReport> {
        let url = format!("/workspaces/{}/scheduled_reports", workspace_id);
        self.client.request_with_body(Method::POST, &url, &report)
    }

    /// Delete a scheduled report
    pub fn delete_scheduled_report(&self, workspace_id: u64, report_id: u64) -> Result<String> {
        let url = format!(
            "/workspaces/{}/scheduled_reports/{}",
            workspace_id, report_id
        );
        self.client.request(Method::DELETE, &url)
    }
}
