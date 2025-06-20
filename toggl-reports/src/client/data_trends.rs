//! Data trends report endpoints

use reqwest::Method;
use toggl_core::{Result, WorkspaceId};

use super::ReportsClient;
use crate::models::data_trends::*;
use crate::models::DataTrendsPost;

pub struct DataTrendsClient {
    client: ReportsClient,
}

impl DataTrendsClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Get client data trends
    pub fn clients(
        &self,
        workspace_id: WorkspaceId,
        request: &DataTrendsPost,
    ) -> Result<ClientDataTrendsReport> {
        let path = format!("/workspace/{}/data_trends/clients", workspace_id.value());
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get project data trends
    pub fn projects(
        &self,
        workspace_id: WorkspaceId,
        request: &DataTrendsPost,
    ) -> Result<ProjectDataTrendsReport> {
        let path = format!("/workspace/{}/data_trends/projects", workspace_id.value());
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get user data trends
    pub fn users(
        &self,
        workspace_id: WorkspaceId,
        request: &DataTrendsPost,
    ) -> Result<UserDataTrendsReport> {
        let path = format!("/workspace/{}/data_trends/users", workspace_id.value());
        self.client.request_with_body(Method::POST, &path, request)
    }
}
