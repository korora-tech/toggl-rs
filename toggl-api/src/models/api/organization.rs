use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use toggl_core::{
    GroupId, OrganizationId, PricingPlanId, RoleId, TransferId, UserId, WorkspaceId,
    WorkspaceUserId,
};

/// Represents a Toggl organization.
///
/// Organizations allow managing multiple workspaces under a single billing entity.
/// They provide centralized user management, billing, and administrative features.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let orgs = client.get_me_organizations().await?;
///
/// for org in orgs {
///     println!("Organization: {} (ID: {})", org.name, org.id);
///     println!("User count: {}", org.user_count);
///     println!("Admin: {}, Owner: {}", org.admin, org.owner);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Organization {
    /// Unique identifier for the organization
    pub id: OrganizationId,
    /// Organization name
    pub name: String,
    /// Current pricing plan
    pub pricing_plan_id: PricingPlanId,
    /// When the organization was created
    pub created_at: DateTime<Utc>,
    /// Server timestamp when this data was retrieved
    pub at: DateTime<Utc>,
    /// When the organization was deleted (if applicable)
    pub server_deleted_at: Option<DateTime<Utc>>,
    /// Whether this is a unified organization
    pub is_unified: bool,
    /// Whether multiple workspaces are enabled
    pub is_multi_workspace_enabled: bool,
    /// Whether using Chargify for billing
    pub is_chargify: bool,
    /// Maximum number of workspaces allowed
    pub max_workspaces: u32,
    /// Whether the current user is an admin
    pub admin: bool,
    /// Whether the current user is the owner
    pub owner: bool,
    /// When the organization was suspended (if applicable)
    pub suspended_at: Option<DateTime<Utc>>,
    /// Total number of users in the organization
    pub user_count: u32,
    /// Trial subscription information
    pub trial_info: TrialInfo,
    /// Available payment methods
    pub payment_methods: Option<String>,
    /// User's permissions in this organization
    pub permissions: Option<Vec<String>>,
}

/// Trial subscription information for an organization.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrialInfo {
    pub trial: bool,
    pub trial_available: bool,
    pub trial_end_date: Option<DateTime<Utc>>,
    pub next_payment_date: Option<DateTime<Utc>>,
    pub last_pricing_plan_id: Option<PricingPlanId>,
}

/// Request structure for creating a new organization.
///
/// Creates an organization with an initial workspace.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrganization {
    pub name: String,
    pub workspace_name: String,
}

/// Request structure for updating organization settings.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOrganization {
    pub name: Option<String>,
}

/// Represents a user within an organization.
///
/// Contains the user's role, permissions, and workspace assignments.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationUser {
    pub id: WorkspaceUserId,
    pub user_id: UserId,
    pub organization_id: OrganizationId,
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

/// Represents a group within an organization.
///
/// Groups are used to manage permissions and access for multiple users.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationGroup {
    pub id: GroupId,
    pub name: String,
    pub workspace_id: WorkspaceId,
}

/// Request structure for creating a new organization group.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrganizationGroup {
    pub name: String,
    pub workspace_id: WorkspaceId,
}

/// Request structure for updating an organization group.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOrganizationGroup {
    pub name: String,
}

/// Represents a workspace within an organization.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationWorkspace {
    pub id: WorkspaceId,
    pub name: String,
    pub admin: bool,
}

/// Information about an organization's owner.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationOwner {
    pub id: UserId,
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
    pub id: WorkspaceUserId,
    pub user_id: UserId,
    pub organization_id: OrganizationId,
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
    pub id: RoleId,
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OwnershipTransferRequest {
    pub id: TransferId,
    pub organization_id: OrganizationId,
    pub current_owner_id: UserId,
    pub new_owner_id: UserId,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOwnershipTransfer {
    pub new_owner_id: UserId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceStatistics {
    pub workspace_id: WorkspaceId,
    pub billable_seconds: i64,
    pub tracked_seconds: i64,
    pub active_member_count: i32,
    pub inactive_member_count: i32,
}
