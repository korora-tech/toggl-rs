use super::time_entry::TimeEntry;
use serde::{Deserialize, Serialize};
use toggl_core::{TimeEntryInvitationId, UserId, WorkspaceId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryInvitation {
    pub time_entry_invitation_id: TimeEntryInvitationId,
    pub workspace_id: WorkspaceId,
    pub shared_by_user_id: UserId,
    pub shared_by_user_name: String,
    pub time_entry: TimeEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InvitationAction {
    Accept,
    Reject,
}
