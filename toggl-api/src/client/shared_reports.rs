use super::TogglClient;
use crate::models::api::shared_reports::{
    BulkDeleteRequest, CreateSavedReportPayload, SavedReport, SharedReportsQuery,
    UpdateSavedReportPayload,
};
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;
use toggl_core::{ReportId, WorkspaceId};

pub struct SharedReportsClient {
    client: TogglClient,
}

impl SharedReportsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get shared reports
    pub fn get_shared_reports(
        &self,
        workspace_id: WorkspaceId,
        query: Option<SharedReportsQuery>,
    ) -> Result<Vec<SavedReport>> {
        let url = format!("/workspaces/{}/reports/shared", workspace_id);
        let mut params = BTreeMap::new();

        if let Some(q) = query {
            if let Some(fixed_dates) = q.fixed_dates {
                params.insert("fixed_dates".to_string(), fixed_dates.to_string());
            }
            if let Some(name) = q.name {
                params.insert("name".to_string(), name);
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
            if let Some(sort_direction) = q.sort_direction {
                params.insert("sort_direction".to_string(), sort_direction);
            }
            if let Some(sort_field) = q.sort_field {
                params.insert("sort_field".to_string(), sort_field);
            }
        }

        self.client.request_with_params(Method::GET, &url, &params)
    }

    /// Create a shared report
    pub fn create_shared_report(
        &self,
        workspace_id: WorkspaceId,
        report: CreateSavedReportPayload,
    ) -> Result<SavedReport> {
        let url = format!("/workspaces/{}/reports/shared", workspace_id);
        self.client.request_with_body(Method::POST, &url, &report)
    }

    /// Update multiple shared reports
    pub fn update_shared_reports(
        &self,
        workspace_id: WorkspaceId,
        reports: Vec<UpdateSavedReportPayload>,
    ) -> Result<SavedReport> {
        let url = format!("/workspaces/{}/reports/shared", workspace_id);
        self.client.request_with_body(Method::PUT, &url, &reports)
    }

    /// Get a specific shared report
    pub fn get_shared_report(
        &self,
        workspace_id: WorkspaceId,
        report_id: ReportId,
    ) -> Result<SavedReport> {
        let url = format!("/workspaces/{}/reports/shared/{}", workspace_id, report_id);
        self.client.request(Method::GET, &url)
    }

    /// Update a specific shared report
    pub fn update_shared_report(
        &self,
        workspace_id: WorkspaceId,
        report_id: ReportId,
        report: UpdateSavedReportPayload,
    ) -> Result<SavedReport> {
        let url = format!("/workspaces/{}/reports/shared/{}", workspace_id, report_id);
        self.client.request_with_body(Method::PUT, &url, &report)
    }

    /// Delete a shared report
    pub fn delete_shared_report(
        &self,
        workspace_id: WorkspaceId,
        report_id: ReportId,
    ) -> Result<SavedReport> {
        let url = format!("/workspaces/{}/reports/shared/{}", workspace_id, report_id);
        self.client.request(Method::DELETE, &url)
    }

    /// Bulk delete shared reports
    pub fn bulk_delete_shared_reports(
        &self,
        workspace_id: WorkspaceId,
        request: BulkDeleteRequest,
    ) -> Result<Vec<SavedReport>> {
        let url = format!("/workspaces/{}/reports/shared/bulk_delete", workspace_id);
        self.client.request_with_body(Method::PATCH, &url, &request)
    }
}
