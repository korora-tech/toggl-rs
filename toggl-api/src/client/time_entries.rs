use super::TogglClient;
use crate::models::api::time_entry::*;
use crate::models::api::time_entry_invitations::{InvitationAction, TimeEntryInvitation};
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;
use toggl_core::{TimeEntryId, TimeEntryInvitationId, WorkspaceId};

pub struct TimeEntriesClient {
    client: TogglClient,
}

impl TimeEntriesClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get the currently running time entry.
    ///
    /// Returns None if no time entry is currently running.
    pub fn current(&self) -> Result<Option<TimeEntry>> {
        self.client.request(Method::GET, "me/time_entries/current")
    }

    /// Get time entries for the current user.
    ///
    /// # Arguments
    /// * `start_date` - Optional start date filter (ISO 8601 format)
    /// * `end_date` - Optional end date filter (ISO 8601 format)
    pub fn list(&self, start_date: Option<&str>, end_date: Option<&str>) -> Result<Vec<TimeEntry>> {
        let mut params = BTreeMap::new();
        if let Some(start) = start_date {
            params.insert("start_date".to_string(), start.to_string());
        }
        if let Some(end) = end_date {
            params.insert("end_date".to_string(), end.to_string());
        }

        self.client
            .request_with_params(Method::GET, "me/time_entries", &params)
    }

    /// Get a specific time entry by ID.
    ///
    /// # Arguments
    /// * `time_entry_id` - The ID of the time entry to retrieve
    pub fn get(&self, time_entry_id: TimeEntryId) -> Result<TimeEntry> {
        self.client
            .request(Method::GET, &format!("me/time_entries/{}", time_entry_id))
    }

    /// Create a new time entry.
    ///
    /// # Arguments
    /// * `workspace_id` - The workspace to create the entry in
    /// * `time_entry` - Time entry creation parameters
    pub fn create(
        &self,
        workspace_id: WorkspaceId,
        time_entry: &CreateTimeEntry,
    ) -> Result<TimeEntry> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/time_entries", workspace_id),
            time_entry,
        )
    }

    /// Start a new time entry (creates and starts it).
    ///
    /// This is a convenience method that creates a time entry with a negative
    /// duration, indicating it's currently running.
    ///
    /// # Arguments
    /// * `workspace_id` - The workspace to create the entry in
    /// * `time_entry` - Time entry creation parameters
    pub fn start(
        &self,
        workspace_id: WorkspaceId,
        time_entry: &CreateTimeEntry,
    ) -> Result<TimeEntry> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/time_entries/start", workspace_id),
            time_entry,
        )
    }

    /// Stop a running time entry.
    ///
    /// # Arguments
    /// * `workspace_id` - The workspace containing the entry
    /// * `time_entry_id` - The ID of the running time entry to stop
    pub fn stop(&self, workspace_id: WorkspaceId, time_entry_id: TimeEntryId) -> Result<TimeEntry> {
        self.client.request(
            Method::PATCH,
            &format!(
                "workspaces/{}/time_entries/{}/stop",
                workspace_id, time_entry_id
            ),
        )
    }

    /// Update an existing time entry.
    ///
    /// # Arguments
    /// * `workspace_id` - The workspace containing the entry
    /// * `time_entry_id` - The ID of the time entry to update
    /// * `time_entry` - Fields to update (all optional)
    pub fn update(
        &self,
        workspace_id: WorkspaceId,
        time_entry_id: TimeEntryId,
        time_entry: &UpdateTimeEntry,
    ) -> Result<TimeEntry> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/time_entries/{}", workspace_id, time_entry_id),
            time_entry,
        )
    }

    /// Delete a time entry.
    ///
    /// # Arguments
    /// * `workspace_id` - The workspace containing the entry
    /// * `time_entry_id` - The ID of the time entry to delete
    pub fn delete(&self, workspace_id: WorkspaceId, time_entry_id: TimeEntryId) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/time_entries/{}", workspace_id, time_entry_id),
        )
    }

    /// Bulk edit time entries
    pub fn bulk_edit(
        &self,
        workspace_id: WorkspaceId,
        bulk_edit: &BulkEditTimeEntries,
    ) -> Result<BulkEditOperation> {
        self.client.request_with_body(
            Method::PATCH,
            &format!("workspaces/{}/time_entries", workspace_id),
            bulk_edit,
        )
    }

    /// Get time entry by ID
    pub fn get_by_id(&self, time_entry_id: TimeEntryId) -> Result<TimeEntry> {
        self.client
            .request(Method::GET, &format!("me/time_entries/{}", time_entry_id))
    }

    /// Bulk delete time entries
    pub fn bulk_delete(
        &self,
        workspace_id: WorkspaceId,
        time_entry_ids: Vec<TimeEntryId>,
    ) -> Result<()> {
        let ids_string = time_entry_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/time_entries/{}", workspace_id, ids_string),
        )
    }

    /// Get time entry checklist
    pub fn get_checklist(&self) -> Result<TimeEntryChecklist> {
        self.client
            .request(Method::GET, "me/time_entries/checklist")
    }

    /// Get invitations for time entries
    pub fn get_invitations(&self, workspace_id: WorkspaceId) -> Result<Vec<TimeEntryInvitation>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/time_entry_invitations", workspace_id),
        )
    }

    /// Accept or reject a time entry invitation
    pub fn respond_to_invitation(
        &self,
        workspace_id: WorkspaceId,
        invitation_id: TimeEntryInvitationId,
        action: InvitationAction,
    ) -> Result<()> {
        let action_str = match action {
            InvitationAction::Accept => "accept",
            InvitationAction::Reject => "reject",
        };
        self.client.request_empty(
            Method::POST,
            &format!(
                "workspaces/{}/time_entry_invitations/{}/{}",
                workspace_id, invitation_id, action_str
            ),
        )
    }
}
