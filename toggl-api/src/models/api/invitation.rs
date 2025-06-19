use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invitation {
    pub id: u64,
    pub email: String,
    pub invitation_id: String,
    pub invite_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub sender_id: u64,
    pub sender_name: String,
    pub sender_email: String,
    pub recipient_id: Option<u64>,
    pub recipient_email: String,
    pub recipient_name: Option<String>,
    pub workspace_id: u64,
    pub workspace_name: String,
    pub organization_id: Option<u64>,
    pub organization_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateInvitation {
    pub workspace_id: u64,
    pub emails: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResendInvitation {
    pub invitation_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AcceptInvitation {
    pub invitation_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SSOInvitation {
    pub id: String,
    pub email: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub workspace_id: u64,
    pub workspace_name: String,
    pub organization_id: Option<u64>,
    pub organization_name: Option<String>,
}
