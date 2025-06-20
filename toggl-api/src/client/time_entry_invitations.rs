use super::TogglClient;
use crate::models::api::time_entry_invitations::TimeEntryInvitation;
use reqwest::Method;
use toggl_core::Result;
use toggl_core::{TimeEntryInvitationId, WorkspaceId};

pub struct TimeEntryInvitationsClient {
    client: TogglClient,
}

impl TimeEntryInvitationsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get time entry invitations for a workspace
    pub fn get_invitations(&self, workspace_id: WorkspaceId) -> Result<Vec<TimeEntryInvitation>> {
        let url = format!("/workspaces/{}/time_entry_invitations", workspace_id);
        self.client.request(Method::GET, &url)
    }

    /// Accept or reject a time entry invitation
    pub fn respond_to_invitation(
        &self,
        workspace_id: WorkspaceId,
        invitation_id: TimeEntryInvitationId,
        action: &str,
    ) -> Result<()> {
        let url = format!(
            "/workspaces/{}/time_entry_invitations/{}/{}",
            workspace_id, invitation_id, action
        );
        self.client.request_empty(Method::POST, &url)
    }

    /// Accept a time entry invitation
    pub fn accept_invitation(
        &self,
        workspace_id: WorkspaceId,
        invitation_id: TimeEntryInvitationId,
    ) -> Result<()> {
        self.respond_to_invitation(workspace_id, invitation_id, "accept")
    }

    /// Reject a time entry invitation
    pub fn reject_invitation(
        &self,
        workspace_id: WorkspaceId,
        invitation_id: TimeEntryInvitationId,
    ) -> Result<()> {
        self.respond_to_invitation(workspace_id, invitation_id, "reject")
    }
}
