use super::TogglClient;
use crate::models::api::project::*;
use crate::models::api::task::Task;
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;
use toggl_core::{GroupId, ProjectId, ProjectUserId, TaskId, WorkspaceId};

pub struct ProjectsClient {
    client: TogglClient,
}

impl ProjectsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get project
    pub fn get(&self, workspace_id: WorkspaceId, project_id: ProjectId) -> Result<Project> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/projects/{}", workspace_id, project_id),
        )
    }

    /// Create project
    pub fn create(&self, workspace_id: WorkspaceId, project: &CreateProject) -> Result<Project> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/projects", workspace_id),
            project,
        )
    }

    /// Update project
    pub fn update(
        &self,
        workspace_id: WorkspaceId,
        project_id: ProjectId,
        project: &UpdateProject,
    ) -> Result<Project> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/projects/{}", workspace_id, project_id),
            project,
        )
    }

    /// Delete project
    pub fn delete(&self, workspace_id: WorkspaceId, project_id: ProjectId) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/projects/{}", workspace_id, project_id),
        )
    }

    /// Get project users
    pub fn get_users(
        &self,
        workspace_id: WorkspaceId,
        project_id: ProjectId,
    ) -> Result<Vec<ProjectUser>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/projects/{}/users", workspace_id, project_id),
        )
    }

    /// Get project statistics
    pub fn get_statistics(
        &self,
        workspace_id: WorkspaceId,
        project_id: ProjectId,
    ) -> Result<ProjectStatistics> {
        self.client.request(
            Method::GET,
            &format!(
                "workspaces/{}/projects/{}/statistics",
                workspace_id, project_id
            ),
        )
    }

    /// Get project tasks
    pub fn get_tasks(&self, workspace_id: WorkspaceId, project_id: ProjectId) -> Result<Vec<Task>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/projects/{}/tasks", workspace_id, project_id),
        )
    }

    /// Create project task
    pub fn create_task(
        &self,
        workspace_id: WorkspaceId,
        project_id: ProjectId,
        name: &str,
        active: bool,
    ) -> Result<Task> {
        let body = serde_json::json!({
            "name": name,
            "active": active,
            "project_id": project_id
        });
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/projects/{}/tasks", workspace_id, project_id),
            &body,
        )
    }

    /// Update project task
    pub fn update_task(
        &self,
        workspace_id: WorkspaceId,
        project_id: ProjectId,
        task_id: TaskId,
        name: &str,
        active: bool,
    ) -> Result<Task> {
        let body = serde_json::json!({
            "name": name,
            "active": active
        });
        self.client.request_with_body(
            Method::PUT,
            &format!(
                "workspaces/{}/projects/{}/tasks/{}",
                workspace_id, project_id, task_id
            ),
            &body,
        )
    }

    /// Delete project task
    pub fn delete_task(
        &self,
        workspace_id: WorkspaceId,
        project_id: ProjectId,
        task_id: TaskId,
    ) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!(
                "workspaces/{}/projects/{}/tasks/{}",
                workspace_id, project_id, task_id
            ),
        )
    }

    // Advanced Project Management

    /// Bulk edit projects
    pub fn bulk_edit(
        &self,
        workspace_id: WorkspaceId,
        project_ids: &[ProjectId],
        operations: &[PatchOperation],
    ) -> Result<Vec<Project>> {
        let ids = project_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.client.request_with_body(
            Method::PATCH,
            &format!("workspaces/{}/projects/{}", workspace_id, ids),
            &operations,
        )
    }

    /// Get project periods
    pub fn get_periods(
        &self,
        workspace_id: WorkspaceId,
        project_id: ProjectId,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Vec<ProjectPeriod>> {
        let mut params = BTreeMap::new();
        if let Some(start) = start_date {
            params.insert("start_date".to_string(), start.to_string());
        }
        if let Some(end) = end_date {
            params.insert("end_date".to_string(), end.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!(
                "workspaces/{}/projects/{}/periods",
                workspace_id, project_id
            ),
            &params,
        )
    }

    /// Pin project
    pub fn pin(&self, workspace_id: WorkspaceId, project_id: ProjectId) -> Result<()> {
        self.client.request_empty(
            Method::POST,
            &format!("workspaces/{}/projects/{}/pin", workspace_id, project_id),
        )
    }

    /// Unpin project
    pub fn unpin(&self, workspace_id: WorkspaceId, project_id: ProjectId) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/projects/{}/pin", workspace_id, project_id),
        )
    }

    /// Bulk edit tasks
    pub fn bulk_edit_tasks(
        &self,
        workspace_id: WorkspaceId,
        project_id: ProjectId,
        task_ids: &[TaskId],
        operations: &[PatchOperation],
    ) -> Result<Vec<Task>> {
        let ids = task_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.client.request_with_body(
            Method::PATCH,
            &format!(
                "workspaces/{}/projects/{}/tasks/{}",
                workspace_id, project_id, ids
            ),
            &operations,
        )
    }

    /// Get billable amounts for projects
    pub fn get_billable_amounts(
        &self,
        workspace_id: WorkspaceId,
        project_ids: &[ProjectId],
    ) -> Result<Vec<Project>> {
        let payload = ProjectIds {
            project_ids: project_ids.to_vec(),
        };

        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/projects/billable-amounts", workspace_id),
            &payload,
        )
    }

    /// Get project task count
    pub fn get_task_count(&self, workspace_id: WorkspaceId) -> Result<serde_json::Value> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/projects/task_count", workspace_id),
        )
    }

    /// Get project templates
    pub fn get_templates(&self, workspace_id: WorkspaceId) -> Result<Vec<ProjectTemplate>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/projects/templates", workspace_id),
        )
    }

    /// Get project user count
    pub fn get_user_count(&self, workspace_id: WorkspaceId) -> Result<serde_json::Value> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/projects/user_count", workspace_id),
        )
    }

    // Project Groups

    /// Get project groups
    pub fn get_groups(&self, workspace_id: WorkspaceId) -> Result<Vec<ProjectGroup>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/project_groups", workspace_id),
        )
    }

    /// Create project group
    pub fn create_group(
        &self,
        workspace_id: WorkspaceId,
        payload: &ProjectGroupPayload,
    ) -> Result<ProjectGroup> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/project_groups", workspace_id),
            payload,
        )
    }

    /// Delete project group
    pub fn delete_group(&self, workspace_id: WorkspaceId, project_group_id: GroupId) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!(
                "workspaces/{}/project_groups/{}",
                workspace_id, project_group_id
            ),
        )
    }

    // Project Users

    /// List project users
    pub fn list_project_users(&self, workspace_id: WorkspaceId) -> Result<Vec<ProjectUser>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/project_users", workspace_id),
        )
    }

    /// List project users paginated
    pub fn list_project_users_paginated(
        &self,
        workspace_id: WorkspaceId,
        page: Option<u32>,
        per_page: Option<u32>,
        sort_field: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<Vec<ProjectUser>> {
        let mut params = BTreeMap::new();
        if let Some(p) = page {
            params.insert("page".to_string(), p.to_string());
        }
        if let Some(pp) = per_page {
            params.insert("per_page".to_string(), pp.to_string());
        }
        if let Some(sf) = sort_field {
            params.insert("sort_field".to_string(), sf.to_string());
        }
        if let Some(so) = sort_order {
            params.insert("sort_order".to_string(), so.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/project_users/paginated", workspace_id),
            &params,
        )
    }

    /// Create project user
    pub fn create_project_user(
        &self,
        workspace_id: WorkspaceId,
        project_user: &CreateProjectUser,
    ) -> Result<ProjectUser> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/project_users", workspace_id),
            project_user,
        )
    }

    /// Bulk edit project users
    pub fn bulk_edit_project_users(
        &self,
        workspace_id: WorkspaceId,
        project_user_ids: &[ProjectUserId],
        operations: &[PatchOperation],
    ) -> Result<Vec<ProjectUser>> {
        let ids = project_user_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.client.request_with_body(
            Method::PATCH,
            &format!("workspaces/{}/project_users/{}", workspace_id, ids),
            &operations,
        )
    }

    /// Update project user
    pub fn update_project_user(
        &self,
        workspace_id: WorkspaceId,
        project_user_id: ProjectUserId,
        update: &UpdateProjectUser,
    ) -> Result<ProjectUser> {
        self.client.request_with_body(
            Method::PUT,
            &format!(
                "workspaces/{}/project_users/{}",
                workspace_id, project_user_id
            ),
            update,
        )
    }

    /// Delete project user
    pub fn delete_project_user(
        &self,
        workspace_id: WorkspaceId,
        project_user_id: ProjectUserId,
    ) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!(
                "workspaces/{}/project_users/{}",
                workspace_id, project_user_id
            ),
        )
    }
}
