use crate::model::api::time_entry::TimeEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryInvitation {
    pub time_entry_invitation_id: i64,
    pub workspace_id: i64,
    pub shared_by_user_id: i64,
    pub shared_by_user_name: String,
    pub time_entry: TimeEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InvitationAction {
    Accept,
    Reject,
}
