use super::TogglClient;
use crate::models::api::client::Client;
use crate::models::api::favorite::{CreateFavorite, Favorite, UpdateFavorite};
use crate::models::api::features::{Features, Location, TrackReminder};
use crate::models::api::organization::Organization;
use crate::models::api::preferences::*;
use crate::models::api::project::Project;
use crate::models::api::task::Task;
use crate::models::api::time_entry::{
    CreateTimeEntry, TimeEntry, TimeEntryChecklist, UpdateTimeEntry,
};
use crate::models::api::timesheets::{TimesheetFilter, TimesheetsGetPaginatedResponse};
use crate::models::api::user::{LostPassword, ResetToken, Tag, UpdateUser, User, UserWebTimer};
use crate::models::api::workspace::Workspace;
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;

pub struct MeClient {
    client: TogglClient,
}

impl MeClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get current user
    pub fn get(&self) -> Result<User> {
        self.client.request(Method::GET, "me")
    }

    /// Update current user
    pub fn update(&self, user_update: &UpdateUser) -> Result<User> {
        self.client
            .request_with_body(Method::PUT, "me", user_update)
    }

    /// Get clients
    pub fn get_clients(&self, since: Option<i64>) -> Result<Vec<Client>> {
        let mut params = BTreeMap::new();
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        self.client
            .request_with_params(Method::GET, "me/clients", &params)
    }

    /// Get features
    pub fn get_features(&self) -> Result<Vec<Features>> {
        self.client.request(Method::GET, "me/features")
    }

    /// Get location
    pub fn get_location(&self) -> Result<Location> {
        self.client.request(Method::GET, "me/location")
    }

    /// Check if logged in
    pub fn get_logged(&self) -> Result<()> {
        self.client.request_empty(Method::GET, "me/logged")
    }

    /// Get organizations user is part of
    pub fn get_organizations(&self) -> Result<Vec<Organization>> {
        self.client.request(Method::GET, "me/organizations")
    }

    /// Get projects
    pub fn get_projects(
        &self,
        include_archived: Option<bool>,
        since: Option<i64>,
    ) -> Result<Vec<Project>> {
        let mut params = BTreeMap::new();
        if let Some(include_archived) = include_archived {
            params.insert("include_archived".to_string(), include_archived.to_string());
        }
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        self.client
            .request_with_params(Method::GET, "me/projects", &params)
    }

    /// Get projects paginated
    pub fn get_projects_paginated(
        &self,
        start_project_id: Option<i64>,
        since: Option<i64>,
    ) -> Result<Vec<Project>> {
        let mut params = BTreeMap::new();
        if let Some(start_project_id) = start_project_id {
            params.insert("start_project_id".to_string(), start_project_id.to_string());
        }
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        self.client
            .request_with_params(Method::GET, "me/projects", &params)
    }

    /// Get tags
    pub fn get_tags(&self, since: Option<i64>) -> Result<Vec<Tag>> {
        let mut params = BTreeMap::new();
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        self.client
            .request_with_params(Method::GET, "me/tags", &params)
    }

    /// Get tasks
    pub fn get_tasks(
        &self,
        include_not_active: Option<bool>,
        since: Option<i64>,
    ) -> Result<Vec<Task>> {
        let mut params = BTreeMap::new();
        if let Some(include_not_active) = include_not_active {
            params.insert(
                "include_not_active".to_string(),
                include_not_active.to_string(),
            );
        }
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        self.client
            .request_with_params(Method::GET, "me/tasks", &params)
    }

    /// Get track reminders
    pub fn get_track_reminders(&self) -> Result<Vec<TrackReminder>> {
        self.client.request(Method::GET, "me/track_reminders")
    }

    /// Get workspaces
    pub fn get_workspaces(&self, since: Option<i64>) -> Result<Vec<Workspace>> {
        let mut params = BTreeMap::new();
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        self.client
            .request_with_params(Method::GET, "me/workspaces", &params)
    }

    /// Get web timer
    pub fn get_web_timer(&self) -> Result<UserWebTimer> {
        self.client.request(Method::GET, "me/web-timer")
    }

    /// Close account
    pub fn close_account(&self) -> Result<()> {
        self.client.request_empty(Method::POST, "me/close_account")
    }

    /// Accept terms of service
    pub fn accept_tos(&self) -> Result<()> {
        self.client.request_empty(Method::GET, "me/accept_tos")
    }

    /// Request data export
    pub fn request_export(&self, export_type: &str) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("export_type".to_string(), export_type.to_string());
        self.client
            .request_with_params(Method::GET, "me/export", &params)
    }

    /// Download export data
    pub fn download_export(&self, uuid: &str) -> Result<Vec<u8>> {
        self.client
            .request_binary(Method::GET, &format!("me/export/data/{}.zip", uuid))
    }

    /// Lost password
    pub fn lost_password(&self, lost_password: &LostPassword) -> Result<()> {
        self.client
            .request_with_body_empty(Method::POST, "me/lost_password", lost_password)
    }

    /// Reset token
    pub fn reset_token(&self, reset_token: &ResetToken) -> Result<User> {
        self.client
            .request_with_body(Method::POST, "me/lost_password/token", reset_token)
    }

    /// Enable SSO
    pub fn enable_sso(&self) -> Result<()> {
        self.client.request_empty(Method::GET, "me/enable_sso")
    }

    /// Disable product emails
    pub fn disable_product_emails(&self, disable_code: &str) -> Result<()> {
        self.client.request_empty(
            Method::POST,
            &format!("me/disable_product_emails/{}", disable_code),
        )
    }

    /// Disable weekly report
    pub fn disable_weekly_report(&self, weekly_report_code: &str) -> Result<()> {
        self.client.request_empty(
            Method::POST,
            &format!("me/disable_weekly_report/{}", weekly_report_code),
        )
    }

    /// Get flags
    pub fn get_flags(&self) -> Result<Flags> {
        self.client.request(Method::GET, "me/flags")
    }

    /// Get preferences
    pub fn get_preferences(&self) -> Result<AllPreferences> {
        self.client.request(Method::GET, "me/preferences")
    }

    /// Update preferences
    pub fn update_preferences(&self, preferences: &AllPreferences) -> Result<AllPreferences> {
        self.client
            .request_with_body(Method::POST, "me/preferences", preferences)
    }

    /// Get client preferences
    pub fn get_client_preferences(&self, client: &str) -> Result<ClientPreferences> {
        self.client
            .request(Method::GET, &format!("me/preferences/{}", client))
    }

    /// Update client preferences
    pub fn update_client_preferences(
        &self,
        client: &str,
        preferences: &ClientPreferences,
    ) -> Result<ClientPreferences> {
        self.client.request_with_body(
            Method::POST,
            &format!("me/preferences/{}", client),
            preferences,
        )
    }

    /// Get push services
    pub fn get_push_services(&self) -> Result<Vec<PushService>> {
        self.client.request(Method::GET, "me/push_services")
    }

    /// Subscribe to push service
    pub fn subscribe_push_service(&self, push_service: &PushService) -> Result<PushService> {
        self.client
            .request_with_body(Method::POST, "me/push_services", push_service)
    }

    /// Unsubscribe from push service
    pub fn unsubscribe_push_service(&self, token: &str) -> Result<()> {
        self.client
            .request_empty(Method::DELETE, &format!("me/push_services/{}", token))
    }

    /// Get quota
    pub fn get_quota(&self) -> Result<Quota> {
        self.client.request(Method::GET, "me/quota")
    }

    /// Get shared time entries
    pub fn get_shared_time_entries(&self) -> Result<Vec<SharedTimeEntry>> {
        self.client
            .request(Method::GET, "me/time_entries_shared_with")
    }

    /// Get time entry constraints
    /// This endpoint is not documented but exists in the API spec
    pub fn get_time_entry_constraints(&self, workspace_id: u64) -> Result<TimeEntryConstraints> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/time_entry_constraints", workspace_id),
        )
    }

    /// Get time entries
    pub fn get_time_entries(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
        before: Option<&str>,
        since: Option<i64>,
        group_similar: Option<bool>,
    ) -> Result<Vec<TimeEntry>> {
        let mut params = BTreeMap::new();
        if let Some(start_date) = start_date {
            params.insert("start_date".to_string(), start_date.to_string());
        }
        if let Some(end_date) = end_date {
            params.insert("end_date".to_string(), end_date.to_string());
        }
        if let Some(before) = before {
            params.insert("before".to_string(), before.to_string());
        }
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        if let Some(group_similar) = group_similar {
            params.insert("group_similar".to_string(), group_similar.to_string());
        }
        self.client
            .request_with_params(Method::GET, "me/time_entries", &params)
    }

    /// Create time entry
    pub fn create_time_entry(&self, time_entry: &CreateTimeEntry) -> Result<TimeEntry> {
        self.client
            .request_with_body(Method::POST, "me/time_entries", time_entry)
    }

    /// Get time entry by ID
    pub fn get_time_entry(&self, time_entry_id: u64) -> Result<TimeEntry> {
        self.client
            .request(Method::GET, &format!("me/time_entries/{}", time_entry_id))
    }

    /// Update time entry
    pub fn update_time_entry(
        &self,
        time_entry_id: u64,
        time_entry: &UpdateTimeEntry,
    ) -> Result<TimeEntry> {
        self.client.request_with_body(
            Method::PUT,
            &format!("me/time_entries/{}", time_entry_id),
            time_entry,
        )
    }

    /// Delete time entry
    pub fn delete_time_entry(&self, time_entry_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("me/time_entries/{}", time_entry_id),
        )
    }

    /// Get time entries checklist
    pub fn get_time_entries_checklist(&self) -> Result<TimeEntryChecklist> {
        self.client
            .request(Method::GET, "me/time_entries/checklist")
    }

    /// Get current time entry
    pub fn get_current_time_entry(&self) -> Result<Option<TimeEntry>> {
        self.client.request(Method::GET, "me/time_entries/current")
    }

    /// Update current time entry
    pub fn update_current_time_entry(&self, time_entry: &UpdateTimeEntry) -> Result<TimeEntry> {
        self.client
            .request_with_body(Method::PUT, "me/time_entries/current", time_entry)
    }

    /// Stop current time entry
    pub fn stop_current_time_entry(&self) -> Result<TimeEntry> {
        self.client.request_with_body(
            Method::DELETE,
            "me/time_entries/current",
            &serde_json::json!({}),
        )
    }

    /// Get timesheets
    pub fn get_timesheets(
        &self,
        params: Option<&TimesheetFilter>,
    ) -> Result<TimesheetsGetPaginatedResponse> {
        match params {
            Some(filter) => {
                let mut query_params = BTreeMap::new();
                if let Some(start_date) = &filter.start_date {
                    query_params.insert("start_date".to_string(), start_date.to_string());
                }
                if let Some(end_date) = &filter.end_date {
                    query_params.insert("end_date".to_string(), end_date.to_string());
                }
                if let Some(member_ids) = &filter.member_ids {
                    query_params.insert(
                        "member_ids".to_string(),
                        member_ids
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(","),
                    );
                }
                if let Some(approver_ids) = &filter.approver_ids {
                    query_params.insert(
                        "approver_ids".to_string(),
                        approver_ids
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(","),
                    );
                }
                if let Some(timesheet_setup_ids) = &filter.timesheet_setup_ids {
                    query_params.insert(
                        "timesheet_setup_ids".to_string(),
                        timesheet_setup_ids
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(","),
                    );
                }
                if let Some(statuses) = &filter.statuses {
                    query_params.insert("statuses".to_string(), statuses.join(","));
                }
                if let Some(page) = filter.page {
                    query_params.insert("page".to_string(), page.to_string());
                }
                if let Some(per_page) = filter.per_page {
                    query_params.insert("per_page".to_string(), per_page.to_string());
                }
                if let Some(sort_order) = &filter.sort_order {
                    query_params.insert("sort_order".to_string(), sort_order.to_string());
                }
                self.client
                    .request_with_params(Method::GET, "me/timesheets", &query_params)
            }
            None => self.client.request(Method::GET, "me/timesheets"),
        }
    }

    /// Get favorites
    pub fn get_favorites(&self, since: Option<i64>) -> Result<Vec<Favorite>> {
        let mut params = BTreeMap::new();
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        self.client
            .request_with_params(Method::GET, "me/favorites", &params)
    }

    /// Create favorite
    pub fn create_favorite(&self, favorite: &CreateFavorite) -> Result<Favorite> {
        self.client
            .request_with_body(Method::POST, "me/favorites", favorite)
    }

    /// Update favorite
    pub fn update_favorite(&self, favorite_id: u64, favorite: &UpdateFavorite) -> Result<Favorite> {
        self.client.request_with_body(
            Method::PUT,
            &format!("me/favorites/{}", favorite_id),
            favorite,
        )
    }

    /// Delete favorite
    pub fn delete_favorite(&self, favorite_id: u64) -> Result<()> {
        self.client
            .request_empty(Method::DELETE, &format!("me/favorites/{}", favorite_id))
    }

    /// Get favorite suggestions
    pub fn get_favorite_suggestions(&self) -> Result<Vec<Favorite>> {
        self.client.request(Method::GET, "me/favorites/suggestions")
    }
}
