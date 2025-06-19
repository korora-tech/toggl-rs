use super::TogglClient;
use crate::error::Result;
use crate::model::api::ApiTag;
use crate::model::api::alert::*;
use crate::model::api::client::{ArchiveClientsResponse, Client};
use crate::model::api::currency::Currency;
use crate::model::api::dashboard::*;
use crate::model::api::expense::{CreateExpense, Expense};
use crate::model::api::exports::DownloadRequestRecord;
use crate::model::api::favorite::{CreateFavorite, Favorite, UpdateFavorite};
use crate::model::api::features::TrackReminder;
use crate::model::api::goals::{
    CreateGoalRequest, Goal as WorkspaceGoalModel, UpdateGoalRequest, WorkspaceGoal,
    WorkspaceGoalsQuery,
};
use crate::model::api::group::{CreateGroup, Group, UpdateGroup};
use crate::model::api::preferences::{Logo, TimeEntryConstraints, WorkspacePreferences};
use crate::model::api::project::{
    CreateProjectUser, PatchOperation, Project, ProjectGroup, ProjectGroupPayload, ProjectUser,
    UpdateProjectUser,
};
use crate::model::api::rates::{CreateRate, Rate, RateLevel};
use crate::model::api::saml::{LinkSsoProfile, LinkedSsoProfile};
use crate::model::api::task::Task;
use crate::model::api::timesheets::{
    APITimesheetSetup, CreateTimesheetSetupPayload, TimesheetSetupsGetPaginatedResponse,
    UpdateTimesheetSetupPayload,
};
use crate::model::api::workspace::*;
use crate::model::api::workspace_subscription::WorkspaceSubscriptionResponse;
use reqwest::Method;
use std::collections::BTreeMap;

pub struct WorkspacesClient {
    client: TogglClient,
}

impl WorkspacesClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get workspace
    pub fn get(&self, workspace_id: u64) -> Result<Workspace> {
        self.client
            .request(Method::GET, &format!("workspaces/{}", workspace_id))
    }

    /// Update workspace
    pub fn update(
        &self,
        workspace_id: u64,
        workspace_update: &UpdateWorkspace,
    ) -> Result<Workspace> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}", workspace_id),
            workspace_update,
        )
    }

    /// Get workspace users
    pub fn get_users(&self, workspace_id: u64) -> Result<Vec<WorkspaceUser>> {
        self.client
            .request(Method::GET, &format!("workspaces/{}/users", workspace_id))
    }

    /// Get workspace clients
    pub fn get_clients(&self, workspace_id: u64, status: Option<&str>) -> Result<Vec<Client>> {
        let mut params = BTreeMap::new();
        if let Some(status) = status {
            params.insert("status".to_string(), status.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/clients", workspace_id),
            &params,
        )
    }

    /// Create workspace client
    pub fn create_client(&self, workspace_id: u64, name: &str) -> Result<Client> {
        let body = serde_json::json!({ "name": name });
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/clients", workspace_id),
            &body,
        )
    }

    /// Update workspace client
    pub fn update_client(&self, workspace_id: u64, client_id: u64, name: &str) -> Result<Client> {
        let body = serde_json::json!({ "name": name });
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/clients/{}", workspace_id, client_id),
            &body,
        )
    }

    /// Delete workspace client
    pub fn delete_client(&self, workspace_id: u64, client_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/clients/{}", workspace_id, client_id),
        )
    }

    /// Archive workspace client
    pub fn archive_client(&self, workspace_id: u64, client_id: u64) -> Result<Client> {
        self.client.request(
            Method::POST,
            &format!("workspaces/{}/clients/{}/archive", workspace_id, client_id),
        )
    }

    /// Restore workspace client
    pub fn restore_client(&self, workspace_id: u64, client_id: u64) -> Result<Client> {
        self.client.request(
            Method::POST,
            &format!("workspaces/{}/clients/{}/restore", workspace_id, client_id),
        )
    }

    /// Get workspace groups
    pub fn get_groups(&self, workspace_id: u64) -> Result<Vec<Group>> {
        self.client
            .request(Method::GET, &format!("workspaces/{}/groups", workspace_id))
    }

    /// Create workspace group
    pub fn create_group(&self, workspace_id: u64, group: &CreateGroup) -> Result<Group> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/groups", workspace_id),
            group,
        )
    }

    /// Get workspace group by ID
    pub fn get_group(&self, workspace_id: u64, group_id: u64) -> Result<Group> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/groups/{}", workspace_id, group_id),
        )
    }

    /// Update workspace group
    pub fn update_group(
        &self,
        workspace_id: u64,
        group_id: u64,
        group: &UpdateGroup,
    ) -> Result<Group> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/groups/{}", workspace_id, group_id),
            group,
        )
    }

    /// Delete workspace group
    pub fn delete_group(&self, workspace_id: u64, group_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/groups/{}", workspace_id, group_id),
        )
    }

    /// Get workspace projects
    pub fn get_projects(
        &self,
        workspace_id: u64,
        active: Option<bool>,
        since: Option<i64>,
    ) -> Result<Vec<Project>> {
        let mut params = BTreeMap::new();
        if let Some(active) = active {
            params.insert("active".to_string(), active.to_string());
        }
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/projects", workspace_id),
            &params,
        )
    }

    /// Get workspace tasks
    pub fn get_tasks(
        &self,
        workspace_id: u64,
        active: Option<bool>,
        since: Option<i64>,
    ) -> Result<Vec<Task>> {
        let mut params = BTreeMap::new();
        if let Some(active) = active {
            params.insert("active".to_string(), active.to_string());
        }
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/tasks", workspace_id),
            &params,
        )
    }

    /// Get workspace tags
    pub fn get_tags(&self, workspace_id: u64) -> Result<Vec<ApiTag>> {
        self.client
            .request(Method::GET, &format!("workspaces/{}/tags", workspace_id))
    }

    /// Create workspace tag
    pub fn create_tag(&self, workspace_id: u64, name: &str) -> Result<ApiTag> {
        let body = serde_json::json!({ "name": name });
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/tags", workspace_id),
            &body,
        )
    }

    /// Update workspace tag
    pub fn update_tag(&self, workspace_id: u64, tag_id: u64, name: &str) -> Result<ApiTag> {
        let body = serde_json::json!({ "name": name });
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/tags/{}", workspace_id, tag_id),
            &body,
        )
    }

    /// Delete workspace tag
    pub fn delete_tag(&self, workspace_id: u64, tag_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/tags/{}", workspace_id, tag_id),
        )
    }

    /// Get workspace statistics
    pub fn get_statistics(&self, workspace_id: u64) -> Result<WorkspaceStatistics> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/statistics", workspace_id),
        )
    }

    /// Create workspace
    pub fn create(&self, workspace_create: &CreateWorkspace) -> Result<Workspace> {
        self.client
            .request_with_body(Method::POST, "workspaces", workspace_create)
    }

    /// Get all activity dashboard
    pub fn get_all_activity(&self, workspace_id: u64) -> Result<AllActivity> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/dashboard/all_activity", workspace_id),
        )
    }

    /// Get most active users dashboard
    pub fn get_most_active(&self, workspace_id: u64) -> Result<MostActive> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/dashboard/most_active", workspace_id),
        )
    }

    /// Get top activities dashboard
    pub fn get_top_activity(&self, workspace_id: u64) -> Result<TopActivities> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/dashboard/top_activity", workspace_id),
        )
    }

    /// Get workspace expenses
    pub fn get_expenses(&self, workspace_id: u64) -> Result<Vec<Expense>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/expenses", workspace_id),
        )
    }

    /// Create workspace expense
    pub fn create_expense(&self, workspace_id: u64, expense: &CreateExpense) -> Result<Expense> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/expenses", workspace_id),
            expense,
        )
    }

    /// Create workspace expense with receipt
    /// This endpoint creates an expense with an attached receipt file
    pub fn create_expense_with_receipt(
        &self,
        workspace_id: u64,
        expense: &CreateExpense,
        receipt_data: &[u8],
        receipt_filename: &str,
    ) -> Result<Expense> {
        // Detect MIME type based on file extension
        let mime_type = match receipt_filename.split('.').next_back() {
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("gif") => "image/gif",
            Some("pdf") => "application/pdf",
            _ => "application/octet-stream",
        };

        self.client.request_multipart_json_file(
            Method::POST,
            &format!("workspaces/{}/expenses", workspace_id),
            expense,
            receipt_data,
            receipt_filename,
            "receipt",
            mime_type,
        )
    }

    /// Get workspace currencies
    pub fn get_currencies(&self, workspace_id: u64) -> Result<Vec<Currency>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/currencies", workspace_id),
        )
    }

    /// Get workspace track reminders
    pub fn get_track_reminders(&self, workspace_id: u64) -> Result<Vec<TrackReminder>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/track_reminders", workspace_id),
        )
    }

    /// Create workspace track reminder
    pub fn create_track_reminder(
        &self,
        workspace_id: u64,
        reminder: &TrackReminder,
    ) -> Result<TrackReminder> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/track_reminders", workspace_id),
            reminder,
        )
    }

    /// Update workspace track reminder
    pub fn update_track_reminder(
        &self,
        workspace_id: u64,
        reminder_id: u64,
        reminder: &TrackReminder,
    ) -> Result<TrackReminder> {
        self.client.request_with_body(
            Method::PUT,
            &format!(
                "workspaces/{}/track_reminders/{}",
                workspace_id, reminder_id
            ),
            reminder,
        )
    }

    /// Delete workspace track reminder
    pub fn delete_track_reminder(&self, workspace_id: u64, reminder_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!(
                "workspaces/{}/track_reminders/{}",
                workspace_id, reminder_id
            ),
        )
    }

    /// Get workspace alerts
    pub fn get_alerts(&self, workspace_id: u64) -> Result<Vec<AlertWithMeta>> {
        self.client
            .request(Method::GET, &format!("workspaces/{}/alerts", workspace_id))
    }

    /// Create workspace alert
    pub fn create_alert(&self, workspace_id: u64, alert: &CreateAlert) -> Result<Alert> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/alerts", workspace_id),
            alert,
        )
    }

    /// Update workspace alert
    pub fn update_alert(
        &self,
        workspace_id: u64,
        alert_id: u64,
        alert: &UpdateAlert,
    ) -> Result<Alert> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/alerts/{}", workspace_id, alert_id),
            alert,
        )
    }

    /// Delete workspace alert
    pub fn delete_alert(&self, workspace_id: u64, alert_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/alerts/{}", workspace_id, alert_id),
        )
    }

    /// Add workspace user
    pub fn add_user(&self, workspace_id: u64, emails: Vec<String>) -> Result<WorkspaceUser> {
        let body = serde_json::json!({ "emails": emails });
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/workspace_users", workspace_id),
            &body,
        )
    }

    /// Update workspace user
    pub fn update_workspace_user(
        &self,
        workspace_id: u64,
        workspace_user_id: u64,
        admin: bool,
    ) -> Result<WorkspaceUser> {
        let body = serde_json::json!({ "admin": admin });
        self.client.request_with_body(
            Method::PUT,
            &format!(
                "workspaces/{}/workspace_users/{}",
                workspace_id, workspace_user_id
            ),
            &body,
        )
    }

    /// Remove workspace user
    pub fn remove_user(&self, workspace_id: u64, workspace_user_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!(
                "workspaces/{}/workspace_users/{}",
                workspace_id, workspace_user_id
            ),
        )
    }

    /// Get workspace export requests
    pub fn get_exports(&self, workspace_id: u64) -> Result<Vec<DownloadRequestRecord>> {
        self.client
            .request(Method::GET, &format!("workspaces/{}/exports", workspace_id))
    }

    /// Create workspace export request
    pub fn create_export(&self, workspace_id: u64, tokens_list: Vec<String>) -> Result<String> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/exports", workspace_id),
            &tokens_list,
        )
    }

    /// Download workspace export data
    pub fn download_export(&self, workspace_id: u64, uuid: &str) -> Result<Vec<u8>> {
        self.client.request_binary(
            Method::GET,
            &format!("workspaces/{}/exports/data/{}.zip", workspace_id, uuid),
        )
    }

    /// Get workspace favorites
    pub fn get_favorites(&self, workspace_id: u64, since: Option<u64>) -> Result<Vec<Favorite>> {
        let mut url = format!("workspaces/{}/favorites", workspace_id);
        if let Some(since_ts) = since {
            url.push_str(&format!("?since={}", since_ts));
        }
        self.client.request(Method::GET, &url)
    }

    /// Create workspace favorite
    pub fn create_favorite(
        &self,
        workspace_id: u64,
        favorite: &CreateFavorite,
    ) -> Result<Favorite> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/favorites", workspace_id),
            favorite,
        )
    }

    /// Update workspace favorites
    pub fn update_favorites(
        &self,
        workspace_id: u64,
        favorites: &Vec<UpdateFavorite>,
    ) -> Result<Vec<Favorite>> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/favorites", workspace_id),
            favorites,
        )
    }

    /// Delete workspace favorite
    pub fn delete_favorite(&self, workspace_id: u64, favorite_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/favorites/{}", workspace_id, favorite_id),
        )
    }

    /// Get workspace favorite suggestions
    pub fn get_favorite_suggestions(&self, workspace_id: u64) -> Result<Vec<Favorite>> {
        self.client.request(
            Method::POST,
            &format!("workspaces/{}/favorites/suggestions", workspace_id),
        )
    }

    /// Create workspace rate
    pub fn create_rate(&self, workspace_id: u64, rate: &CreateRate) -> Result<()> {
        self.client.request_with_body_empty(
            Method::POST,
            &format!("workspaces/{}/rates", workspace_id),
            rate,
        )
    }

    /// Get workspace rates by level
    pub fn get_rates(
        &self,
        workspace_id: u64,
        level: &RateLevel,
        level_id: u64,
        rate_type: Option<&str>,
    ) -> Result<Vec<Rate>> {
        let mut url = format!("workspaces/{}/rates/{}/{}", workspace_id, level, level_id);
        if let Some(rt) = rate_type {
            url.push_str(&format!("?type={}", rt));
        }
        self.client.request(Method::GET, &url)
    }

    /// Archive clients in bulk
    pub fn archive_clients_bulk(
        &self,
        workspace_id: u64,
        client_ids: Vec<u64>,
    ) -> Result<ArchiveClientsResponse> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/clients/archive", workspace_id),
            &client_ids,
        )
    }

    /// Get clients data by IDs
    pub fn get_clients_data(&self, workspace_id: u64, client_ids: Vec<u64>) -> Result<Vec<Client>> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/clients/data", workspace_id),
            &client_ids,
        )
    }

    /// Delete clients in bulk
    pub fn delete_clients_bulk(&self, workspace_id: u64, client_ids: Vec<u64>) -> Result<()> {
        self.client.request_with_body_empty(
            Method::POST,
            &format!("workspaces/{}/clients/delete", workspace_id),
            &client_ids,
        )
    }

    /// Get workspace preferences
    pub fn get_preferences(&self, workspace_id: u64) -> Result<Logo> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/preferences", workspace_id),
        )
    }

    /// Update workspace preferences
    pub fn update_preferences(
        &self,
        workspace_id: u64,
        preferences: &WorkspacePreferences,
    ) -> Result<Logo> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/preferences", workspace_id),
            preferences,
        )
    }

    /// Get workspace subscription
    pub fn get_subscription(&self, workspace_id: u64) -> Result<WorkspaceSubscriptionResponse> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/subscription", workspace_id),
        )
    }

    /// Get purchase order PDF
    pub fn get_purchase_order_pdf(
        &self,
        workspace_id: u64,
        purchase_order_id: u64,
    ) -> Result<Vec<u8>> {
        self.client.request_binary(
            Method::GET,
            &format!(
                "workspaces/{}/subscription/purchase_orders/{}.pdf",
                workspace_id, purchase_order_id
            ),
        )
    }

    /// Get all goals for the requesting user in the workspace
    pub fn get_goals(
        &self,
        workspace_id: u64,
        query: Option<&WorkspaceGoalsQuery>,
    ) -> Result<Vec<WorkspaceGoal>> {
        let mut params = BTreeMap::new();

        if let Some(q) = query {
            if let Some(team_goals) = q.team_goals {
                params.insert("team_goals".to_string(), team_goals.to_string());
            }
            if let Some(active) = q.active {
                params.insert("active".to_string(), active.to_string());
            }
            if let Some(page) = q.page {
                params.insert("page".to_string(), page.to_string());
            }
            if let Some(per_page) = q.per_page {
                params.insert("per_page".to_string(), per_page.to_string());
            }
        }

        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/goals", workspace_id),
            &params,
        )
    }

    /// Create a goal
    pub fn create_goal(
        &self,
        workspace_id: u64,
        goal: &CreateGoalRequest,
    ) -> Result<WorkspaceGoalModel> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/goals", workspace_id),
            goal,
        )
    }

    /// Get one goal
    pub fn get_goal(&self, workspace_id: u64, goal_id: u64) -> Result<WorkspaceGoal> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/goals/{}", workspace_id, goal_id),
        )
    }

    /// Update a goal
    pub fn update_goal(
        &self,
        workspace_id: u64,
        goal_id: u64,
        goal: &UpdateGoalRequest,
    ) -> Result<WorkspaceGoalModel> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/goals/{}", workspace_id, goal_id),
            goal,
        )
    }

    /// Delete a goal
    pub fn delete_goal(&self, workspace_id: u64, goal_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/goals/{}", workspace_id, goal_id),
        )
    }

    /// Get project groups
    pub fn get_project_groups(&self, workspace_id: u64) -> Result<Vec<ProjectGroup>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/project_groups", workspace_id),
        )
    }

    /// Create project group
    pub fn create_project_group(
        &self,
        workspace_id: u64,
        project_group: &ProjectGroupPayload,
    ) -> Result<ProjectGroup> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/project_groups", workspace_id),
            project_group,
        )
    }

    /// Update project group
    pub fn update_project_group(
        &self,
        workspace_id: u64,
        project_group_id: u64,
        project_group: &ProjectGroupPayload,
    ) -> Result<ProjectGroup> {
        self.client.request_with_body(
            Method::PUT,
            &format!(
                "workspaces/{}/project_groups/{}",
                workspace_id, project_group_id
            ),
            project_group,
        )
    }

    /// Delete project group
    pub fn delete_project_group(&self, workspace_id: u64, project_group_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!(
                "workspaces/{}/project_groups/{}",
                workspace_id, project_group_id
            ),
        )
    }

    /// Get project users
    pub fn get_project_users(&self, workspace_id: u64) -> Result<Vec<ProjectUser>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/project_users", workspace_id),
        )
    }

    /// Get paginated project users
    #[allow(clippy::too_many_arguments)]
    pub fn get_project_users_paginated(
        &self,
        workspace_id: u64,
        page: Option<u32>,
        per_page: Option<u32>,
        sort_field: Option<&str>,
        sort_order: Option<&str>,
        only_templates: Option<bool>,
        only_active: Option<bool>,
        project_ids: Option<Vec<u64>>,
        user_ids: Option<Vec<u64>>,
    ) -> Result<Vec<ProjectUser>> {
        let mut params = BTreeMap::new();
        if let Some(page) = page {
            params.insert("page".to_string(), page.to_string());
        }
        if let Some(per_page) = per_page {
            params.insert("per_page".to_string(), per_page.to_string());
        }
        if let Some(sort_field) = sort_field {
            params.insert("sort_field".to_string(), sort_field.to_string());
        }
        if let Some(sort_order) = sort_order {
            params.insert("sort_order".to_string(), sort_order.to_string());
        }
        if let Some(only_templates) = only_templates {
            params.insert("only_templates".to_string(), only_templates.to_string());
        }
        if let Some(only_active) = only_active {
            params.insert("only_active".to_string(), only_active.to_string());
        }
        if let Some(project_ids) = project_ids {
            params.insert(
                "project_ids".to_string(),
                project_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(user_ids) = user_ids {
            params.insert(
                "user_ids".to_string(),
                user_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
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
        workspace_id: u64,
        project_user: &CreateProjectUser,
    ) -> Result<ProjectUser> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/project_users", workspace_id),
            project_user,
        )
    }

    /// Get project user
    pub fn get_project_user(&self, workspace_id: u64, project_user_id: u64) -> Result<ProjectUser> {
        self.client.request(
            Method::GET,
            &format!(
                "workspaces/{}/project_users/{}",
                workspace_id, project_user_id
            ),
        )
    }

    /// Update project user
    pub fn update_project_user(
        &self,
        workspace_id: u64,
        project_user_id: u64,
        project_user: &UpdateProjectUser,
    ) -> Result<ProjectUser> {
        self.client.request_with_body(
            Method::PUT,
            &format!(
                "workspaces/{}/project_users/{}",
                workspace_id, project_user_id
            ),
            project_user,
        )
    }

    /// Delete project user
    pub fn delete_project_user(&self, workspace_id: u64, project_user_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!(
                "workspaces/{}/project_users/{}",
                workspace_id, project_user_id
            ),
        )
    }

    /// Update multiple project users
    pub fn update_project_users_batch(
        &self,
        workspace_id: u64,
        project_user_ids: &[u64],
        operations: &[PatchOperation],
    ) -> Result<Vec<ProjectUser>> {
        let path = format!(
            "workspaces/{}/project_users/{}",
            workspace_id,
            project_user_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        self.client
            .request_with_body(Method::PATCH, &path, &operations)
    }

    /// Get linked SSO profiles
    pub fn get_linked_sso_profiles(&self, workspace_id: u64) -> Result<Vec<LinkedSsoProfile>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/linked_sso_profiles", workspace_id),
        )
    }

    /// Link SSO profile
    pub fn link_sso_profile(
        &self,
        workspace_id: u64,
        link_profile: &LinkSsoProfile,
    ) -> Result<LinkedSsoProfile> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/linked_sso_profiles", workspace_id),
            link_profile,
        )
    }

    /// Get linked SSO profile
    pub fn get_linked_sso_profile(
        &self,
        workspace_id: u64,
        sso_profile_id: u64,
    ) -> Result<LinkedSsoProfile> {
        self.client.request(
            Method::GET,
            &format!(
                "workspaces/{}/linked_sso_profiles/{}",
                workspace_id, sso_profile_id
            ),
        )
    }

    /// Update linked SSO profile
    pub fn update_linked_sso_profile(
        &self,
        workspace_id: u64,
        sso_profile_id: u64,
        link_profile: &LinkSsoProfile,
    ) -> Result<LinkedSsoProfile> {
        self.client.request_with_body(
            Method::PUT,
            &format!(
                "workspaces/{}/linked_sso_profiles/{}",
                workspace_id, sso_profile_id
            ),
            link_profile,
        )
    }

    /// Delete linked SSO profile
    pub fn delete_linked_sso_profile(&self, workspace_id: u64, sso_profile_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!(
                "workspaces/{}/linked_sso_profiles/{}",
                workspace_id, sso_profile_id
            ),
        )
    }

    /// Get workspace logo
    pub fn get_logo(&self, workspace_id: u64) -> Result<Vec<u8>> {
        self.client
            .request_binary(Method::GET, &format!("workspaces/{}/logo", workspace_id))
    }

    /// Upload workspace logo
    pub fn upload_logo(&self, workspace_id: u64, logo_data: &[u8]) -> Result<()> {
        // Detect MIME type based on file data (simple detection)
        let mime_type = if logo_data.len() >= 4 {
            match &logo_data[0..4] {
                [0x89, 0x50, 0x4E, 0x47] => "image/png",
                [0xFF, 0xD8, 0xFF, _] => "image/jpeg",
                [0x47, 0x49, 0x46, _] => "image/gif",
                _ => "image/png", // default
            }
        } else {
            "image/png"
        };

        self.client.request_multipart_with_mime(
            Method::POST,
            &format!("workspaces/{}/logo", workspace_id),
            logo_data,
            "logo.png",
            mime_type,
        )
    }

    /// Update workspace logo
    pub fn update_logo(&self, workspace_id: u64, logo_data: &[u8]) -> Result<()> {
        // Detect MIME type based on file data (simple detection)
        let mime_type = if logo_data.len() >= 4 {
            match &logo_data[0..4] {
                [0x89, 0x50, 0x4E, 0x47] => "image/png",
                [0xFF, 0xD8, 0xFF, _] => "image/jpeg",
                [0x47, 0x49, 0x46, _] => "image/gif",
                _ => "image/png", // default
            }
        } else {
            "image/png"
        };

        self.client.request_multipart_with_mime(
            Method::PUT,
            &format!("workspaces/{}/logo", workspace_id),
            logo_data,
            "logo.png",
            mime_type,
        )
    }

    /// Get payment receipt PDF
    pub fn get_payment_receipt_pdf(&self, workspace_id: u64, payment_id: &str) -> Result<Vec<u8>> {
        self.client.request_binary(
            Method::GET,
            &format!(
                "workspaces/{}/payment_receipts/{}.pdf",
                workspace_id, payment_id
            ),
        )
    }

    /// Upload expense
    pub fn upload_expense(&self, workspace_id: u64, expense_data: &[u8]) -> Result<()> {
        self.client.request_multipart(
            Method::POST,
            &format!("workspaces/{}/expenses/upload", workspace_id),
            expense_data,
            "expenses.csv",
        )
    }

    /// Get timesheet setups
    #[allow(clippy::too_many_arguments)]
    pub fn get_timesheet_setups(
        &self,
        workspace_id: u64,
        member_ids: Option<&[u64]>,
        approver_ids: Option<&[u64]>,
        page: Option<u32>,
        per_page: Option<u32>,
        sort_field: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<TimesheetSetupsGetPaginatedResponse> {
        let mut params = BTreeMap::new();
        if let Some(member_ids) = member_ids {
            params.insert(
                "member_ids".to_string(),
                member_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(approver_ids) = approver_ids {
            params.insert(
                "approver_ids".to_string(),
                approver_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(page) = page {
            params.insert("page".to_string(), page.to_string());
        }
        if let Some(per_page) = per_page {
            params.insert("per_page".to_string(), per_page.to_string());
        }
        if let Some(sort_field) = sort_field {
            params.insert("sort_field".to_string(), sort_field.to_string());
        }
        if let Some(sort_order) = sort_order {
            params.insert("sort_order".to_string(), sort_order.to_string());
        }
        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/timesheet_setups", workspace_id),
            &params,
        )
    }

    /// Create timesheet setup
    pub fn create_timesheet_setup(
        &self,
        workspace_id: u64,
        setup: &CreateTimesheetSetupPayload,
    ) -> Result<Vec<APITimesheetSetup>> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/timesheet_setups", workspace_id),
            setup,
        )
    }

    /// Get timesheet setup
    pub fn get_timesheet_setup(
        &self,
        workspace_id: u64,
        setup_id: u64,
    ) -> Result<APITimesheetSetup> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/timesheet_setups/{}", workspace_id, setup_id),
        )
    }

    /// Update timesheet setup
    pub fn update_timesheet_setup(
        &self,
        workspace_id: u64,
        setup_id: u64,
        setup: &UpdateTimesheetSetupPayload,
    ) -> Result<APITimesheetSetup> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/timesheet_setups/{}", workspace_id, setup_id),
            setup,
        )
    }

    /// Delete timesheet setup
    pub fn delete_timesheet_setup(&self, workspace_id: u64, setup_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/timesheet_setups/{}", workspace_id, setup_id),
        )
    }

    // Dashboard endpoints

    /// Get all activity dashboard
    pub fn get_dashboard_all_activity(
        &self,
        workspace_id: u64,
        period: Option<&str>,
    ) -> Result<AllActivity> {
        let mut params = BTreeMap::new();
        if let Some(period) = period {
            params.insert("period".to_string(), period.to_string());
        }
        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/dashboard/all_activity", workspace_id),
            &params,
        )
    }

    /// Get most active users dashboard
    pub fn get_dashboard_most_active(
        &self,
        workspace_id: u64,
        period: Option<&str>,
    ) -> Result<MostActive> {
        let mut params = BTreeMap::new();
        if let Some(period) = period {
            params.insert("period".to_string(), period.to_string());
        }
        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/dashboard/most_active", workspace_id),
            &params,
        )
    }

    /// Get top activities dashboard
    pub fn get_dashboard_top_activity(
        &self,
        workspace_id: u64,
        period: Option<&str>,
    ) -> Result<TopActivities> {
        let mut params = BTreeMap::new();
        if let Some(period) = period {
            params.insert("period".to_string(), period.to_string());
        }
        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/dashboard/top_activity", workspace_id),
            &params,
        )
    }

    // Time Entry Constraints endpoints

    /// Get time entry constraints
    pub fn get_time_entry_constraints(&self, workspace_id: u64) -> Result<TimeEntryConstraints> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/time_entry_constraints", workspace_id),
        )
    }

    /// Create or update time entry constraints
    pub fn create_time_entry_constraints(
        &self,
        workspace_id: u64,
        constraints: &TimeEntryConstraints,
    ) -> Result<TimeEntryConstraints> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/time_entry_constraints", workspace_id),
            constraints,
        )
    }

    /// Update time entry constraints (PATCH)
    pub fn update_time_entry_constraints(
        &self,
        workspace_id: u64,
        constraints: &TimeEntryConstraints,
    ) -> Result<TimeEntryConstraints> {
        self.client.request_with_body(
            Method::PATCH,
            &format!("workspaces/{}/time_entry_constraints", workspace_id),
            constraints,
        )
    }
}
