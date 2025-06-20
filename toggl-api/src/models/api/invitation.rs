use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::ids::{InvitationId, InvitationItemId, OrganizationId, UserId, WorkspaceId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invitation {
    pub id: InvitationItemId,
    pub email: String,
    pub invitation_id: InvitationId,
    pub invite_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub sender_id: UserId,
    pub sender_name: String,
    pub sender_email: String,
    pub recipient_id: Option<UserId>,
    pub recipient_email: String,
    pub recipient_name: Option<String>,
    pub workspace_id: WorkspaceId,
    pub workspace_name: String,
    pub organization_id: Option<OrganizationId>,
    pub organization_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateInvitation {
    pub workspace_id: WorkspaceId,
    pub emails: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResendInvitation {
    pub invitation_id: InvitationId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AcceptInvitation {
    pub invitation_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SSOInvitation {
    pub id: InvitationId,
    pub email: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub workspace_id: WorkspaceId,
    pub workspace_name: String,
    pub organization_id: Option<OrganizationId>,
    pub organization_name: Option<String>,
}
