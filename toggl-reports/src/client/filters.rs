//! Filter endpoints for reports

use reqwest::Method;
use toggl_core::{Result, WorkspaceId};

use super::ReportsClient;
use crate::models::filters::*;

pub struct FiltersClient {
    client: ReportsClient,
}

impl FiltersClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Get filtered clients
    pub fn clients(
        &self,
        workspace_id: WorkspaceId,
        request: &ClientFilterParamsRequest,
    ) -> Result<Vec<ClientFilterResponse>> {
        let path = format!("/workspace/{}/filters/clients", workspace_id.value());
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get filtered projects
    pub fn projects(
        &self,
        workspace_id: WorkspaceId,
        request: &ProjectFilterParamRequest,
    ) -> Result<Vec<ProjectFilterResponse>> {
        let path = format!("/workspace/{}/filters/projects", workspace_id.value());
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get filtered users
    pub fn users(
        &self,
        workspace_id: WorkspaceId,
        request: &UserFilterParamsRequest,
    ) -> Result<Vec<UserFilterResponse>> {
        let path = format!("/workspace/{}/filters/users", workspace_id.value());
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get project groups
    pub fn project_groups(
        &self,
        workspace_id: WorkspaceId,
        request: &ProjectGroupParamsRequest,
    ) -> Result<Vec<ProjectGroupResponse>> {
        let path = format!("/workspace/{}/filters/project_groups", workspace_id.value());
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get project users
    pub fn project_users(
        &self,
        workspace_id: WorkspaceId,
        request: &ProjectUserParamsRequest,
    ) -> Result<Vec<ProjectUserResponse>> {
        let path = format!("/workspace/{}/filters/project_users", workspace_id.value());
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get project status
    pub fn project_status(
        &self,
        workspace_id: WorkspaceId,
        request: &ProjectStatusParamsRequest,
    ) -> Result<Vec<ProjectStatusResponse>> {
        let path = format!(
            "/workspace/{}/filters/projects/status",
            workspace_id.value()
        );
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get task status
    pub fn task_status(
        &self,
        workspace_id: WorkspaceId,
        request: &TaskStatusParamsRequest,
    ) -> Result<Vec<TaskStatusResponse>> {
        let path = format!("/workspace/{}/filters/tasks/status", workspace_id.value());
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get tasks with specified action (search, filters)
    pub fn tasks(
        &self,
        workspace_id: WorkspaceId,
        action: &str,
        request: &TasksRequest,
    ) -> Result<Vec<Task>> {
        let path = format!("/workspace/{}/{}/tasks", workspace_id.value(), action);
        self.client.request_with_body(Method::POST, &path, request)
    }
}
