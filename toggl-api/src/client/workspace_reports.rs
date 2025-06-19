use super::TogglClient;
use crate::models::api::scheduled_reports::*;
use crate::models::api::shared_reports::*;
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;

pub struct WorkspaceReportsClient {
    client: TogglClient,
}

impl WorkspaceReportsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get shared reports
    pub fn get_shared_reports(
        &self,
        workspace_id: u64,
        query: Option<&SharedReportsQuery>,
    ) -> Result<Vec<SavedReport>> {
        let mut params = BTreeMap::new();

        if let Some(q) = query {
            if let Some(fixed_dates) = q.fixed_dates {
                params.insert("fixed_dates".to_string(), fixed_dates.to_string());
            }
            if let Some(name) = &q.name {
                params.insert("name".to_string(), name.clone());
            }
            if let Some(page) = q.page {
                params.insert("page".to_string(), page.to_string());
            }
            if let Some(per_page) = q.per_page {
                params.insert("per_page".to_string(), per_page.to_string());
            }
            if let Some(public) = q.public {
                params.insert("public".to_string(), public.to_string());
            }
            if let Some(requesting_user_id) = q.requesting_user_id {
                params.insert(
                    "requestingUserID".to_string(),
                    requesting_user_id.to_string(),
                );
            }
            if let Some(scheduled) = q.scheduled {
                params.insert("scheduled".to_string(), scheduled.to_string());
            }
            if let Some(sort_direction) = &q.sort_direction {
                params.insert("sort_direction".to_string(), sort_direction.clone());
            }
            if let Some(sort_field) = &q.sort_field {
                params.insert("sort_field".to_string(), sort_field.clone());
            }
        }

        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/reports/shared", workspace_id),
            &params,
        )
    }

    /// Create a shared report
    pub fn create_shared_report(
        &self,
        workspace_id: u64,
        report: &CreateSavedReportPayload,
    ) -> Result<SavedReport> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/reports/shared", workspace_id),
            report,
        )
    }

    /// Update shared reports
    pub fn update_shared_reports(
        &self,
        workspace_id: u64,
        reports: &[UpdateSavedReportPayload],
    ) -> Result<SavedReport> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/reports/shared", workspace_id),
            &reports,
        )
    }

    /// Get a specific shared report
    pub fn get_shared_report(&self, workspace_id: u64, report_id: u64) -> Result<SavedReport> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/reports/shared/{}", workspace_id, report_id),
        )
    }

    /// Update a specific shared report
    pub fn update_shared_report(
        &self,
        workspace_id: u64,
        report_id: u64,
        report: &UpdateSavedReportPayload,
    ) -> Result<SavedReport> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/reports/shared/{}", workspace_id, report_id),
            report,
        )
    }

    /// Delete a specific shared report
    pub fn delete_shared_report(&self, workspace_id: u64, report_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/reports/shared/{}", workspace_id, report_id),
        )
    }

    /// Bulk delete shared reports
    pub fn bulk_delete_shared_reports(&self, workspace_id: u64, ids: Vec<i64>) -> Result<()> {
        let request = BulkDeleteRequest { ids };
        self.client.request_with_body_empty(
            Method::PATCH,
            &format!("workspaces/{}/reports/shared/bulk_delete", workspace_id),
            &request,
        )
    }

    /// Get scheduled reports
    pub fn get_scheduled_reports(&self, workspace_id: u64) -> Result<Vec<ScheduledReport>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/scheduled_reports", workspace_id),
        )
    }

    /// Create a scheduled report
    pub fn create_scheduled_report(
        &self,
        workspace_id: u64,
        report: &CreateScheduledReportPayload,
    ) -> Result<ScheduledReport> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/scheduled_reports", workspace_id),
            report,
        )
    }

    /// Get a specific scheduled report
    pub fn get_scheduled_report(
        &self,
        workspace_id: u64,
        report_id: u64,
    ) -> Result<ScheduledReport> {
        self.client.request(
            Method::GET,
            &format!(
                "workspaces/{}/scheduled_reports/{}",
                workspace_id, report_id
            ),
        )
    }

    /// Delete a scheduled report
    pub fn delete_scheduled_report(&self, workspace_id: u64, report_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!(
                "workspaces/{}/scheduled_reports/{}",
                workspace_id, report_id
            ),
        )
    }
}
