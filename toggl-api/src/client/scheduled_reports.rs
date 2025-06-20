use super::TogglClient;
use crate::models::api::ids::WorkspaceId;
use crate::models::api::scheduled_reports::{CreateScheduledReportPayload, ScheduledReport};
use reqwest::Method;
use toggl_core::Result;

pub struct ScheduledReportsClient {
    client: TogglClient,
}

impl ScheduledReportsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// List scheduled reports
    pub fn list_scheduled_reports(
        &self,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<ScheduledReport>> {
        let url = format!("/workspaces/{}/scheduled_reports", workspace_id);
        self.client.request(Method::GET, &url)
    }

    /// Create a scheduled report
    pub fn create_scheduled_report(
        &self,
        workspace_id: WorkspaceId,
        report: CreateScheduledReportPayload,
    ) -> Result<ScheduledReport> {
        let url = format!("/workspaces/{}/scheduled_reports", workspace_id);
        self.client.request_with_body(Method::POST, &url, &report)
    }

    /// Delete a scheduled report
    pub fn delete_scheduled_report(
        &self,
        workspace_id: WorkspaceId,
        report_id: u64,
    ) -> Result<String> {
        let url = format!(
            "/workspaces/{}/scheduled_reports/{}",
            workspace_id, report_id
        );
        self.client.request(Method::DELETE, &url)
    }
}
