//! Project-specific report endpoints

use reqwest::Method;
use toggl_core::Result;

use super::ReportsClient;
use crate::models::projects::*;

pub struct ProjectsClient {
    client: ReportsClient,
}

impl ProjectsClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Get summary for all projects in a workspace
    pub fn summary(
        &self,
        workspace_id: i64,
        request: &ProjectSummaryRequest,
    ) -> Result<ProjectsSummaryResponse> {
        let path = format!("/workspace/{}/projects/summary", workspace_id);
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get summary for a specific project
    pub fn project_summary(
        &self,
        workspace_id: i64,
        project_id: i64,
        request: &ProjectSummaryRequest,
    ) -> Result<ProjectSummaryResponse> {
        let path = format!(
            "/workspace/{}/projects/{}/summary",
            workspace_id, project_id
        );
        self.client.request_with_body(Method::POST, &path, request)
    }
}
