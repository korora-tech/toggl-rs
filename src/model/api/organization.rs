use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Organization {
    pub id: u64,
    pub name: String,
    pub pricing_plan_id: u32,
    pub created_at: DateTime<Utc>,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
    pub is_unified: bool,
    pub is_multi_workspace_enabled: bool,
    pub is_chargify: bool,
    pub max_workspaces: u32,
    pub admin: bool,
    pub owner: bool,
    pub suspended_at: Option<DateTime<Utc>>,
    pub user_count: u32,
    pub trial_info: TrialInfo,
    pub payment_methods: Option<String>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrialInfo {
    pub trial: bool,
    pub trial_available: bool,
    pub trial_end_date: Option<DateTime<Utc>>,
    pub next_payment_date: Option<DateTime<Utc>>,
    pub last_pricing_plan_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrganization {
    pub name: String,
    pub workspace_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOrganization {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationUser {
    pub id: u64,
    pub user_id: u64,
    pub organization_id: u64,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub owner: bool,
    pub active: bool,
    pub avatar_file_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub groups: Option<Vec<OrganizationGroup>>,
    pub workspaces: Option<Vec<OrganizationWorkspace>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationGroup {
    pub id: u64,
    pub name: String,
    pub workspace_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrganizationGroup {
    pub name: String,
    pub workspace_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOrganizationGroup {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationWorkspace {
    pub id: u64,
    pub name: String,
    pub admin: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationOwner {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOrganizationUser {
    pub admin: Option<bool>,
    pub owner: Option<bool>,
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationUserDetailed {
    pub id: u64,
    pub user_id: u64,
    pub organization_id: u64,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub owner: bool,
    pub active: bool,
    pub avatar_file_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub groups: Option<Vec<OrganizationGroup>>,
    pub workspaces: Option<Vec<OrganizationWorkspace>>,
    pub joined: Option<bool>,
    pub inactive: Option<bool>,
    pub is_direct: Option<bool>,
    pub labour_cost: Option<f64>,
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationRole {
    pub id: u64,
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OwnershipTransferRequest {
    pub id: u64,
    pub organization_id: u64,
    pub current_owner_id: u64,
    pub new_owner_id: u64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOwnershipTransfer {
    pub new_owner_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceStatistics {
    pub workspace_id: u64,
    pub billable_seconds: i64,
    pub tracked_seconds: i64,
    pub active_member_count: i32,
    pub inactive_member_count: i32,
}
