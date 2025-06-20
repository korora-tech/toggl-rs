use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use toggl_core::{
    CompanyId, CountryId, CustomerId, GroupId, OrganizationId, PricingPlanId, SubscriptionId,
    UserId, WorkspaceId, WorkspaceUserId,
};

/// Represents a Toggl workspace.
///
/// A workspace is the primary organizational unit in Toggl where teams collaborate
/// on time tracking. It contains projects, clients, tags, and team members.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspaces = client.get_workspaces().await?;
///
/// for workspace in workspaces {
///     println!("Workspace: {} (ID: {})", workspace.name, workspace.id);
///     println!("Premium: {}", workspace.premium);
///     println!("Admin: {}", workspace.admin);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workspace {
    /// Unique identifier for the workspace
    pub id: WorkspaceId,
    /// Organization this workspace belongs to (if part of an organization)
    pub organization_id: Option<OrganizationId>,
    /// Name of the workspace
    pub name: String,
    /// Workspace profile type (0 = free, higher values indicate paid plans)
    pub profile: u64,
    /// Whether this is a premium workspace
    pub premium: bool,
    /// Whether this is a business workspace
    pub business_ws: bool,
    /// Whether the current user is an admin of this workspace
    pub admin: bool,
    /// Default hourly rate for new projects in this workspace
    pub default_hourly_rate: Option<f64>,
    /// Default currency code (ISO 4217) for this workspace
    pub default_currency: String,
    /// Whether only admins can create projects
    pub only_admins_may_create_projects: bool,
    /// Whether only admins can create tags
    pub only_admins_may_create_tags: bool,
    /// Whether only admins can see billable rates
    pub only_admins_see_billable_rates: bool,
    /// Whether only admins can see the team dashboard
    pub only_admins_see_team_dashboard: bool,
    /// Whether new projects are billable by default
    pub projects_billable_by_default: bool,
    /// Whether billable setting is enforced for all projects
    pub projects_enforce_billable: bool,
    /// Whether new projects are private by default
    pub projects_private_by_default: bool,
    /// When hourly rates were last updated
    pub rate_last_updated: Option<DateTime<Utc>>,
    /// Whether to collapse reports in the UI
    pub reports_collapse: bool,
    /// Time rounding setting (-1 = round down, 0 = no rounding, 1 = round up)
    pub rounding: i32,
    /// Minutes to round to (e.g., 5, 10, 15)
    pub rounding_minutes: i32,
    /// Workspace API token (deprecated, use user tokens instead)
    pub api_token: Option<String>,
    /// Server timestamp when this data was retrieved
    pub at: DateTime<Utc>,
    /// Whether iCal feed is enabled for this workspace
    pub ical_enabled: bool,
    /// URL for the iCal feed if enabled
    pub ical_url: Option<String>,
    /// Information about CSV uploads to this workspace
    pub csv_upload: Option<CSVUpload>,
    /// Subscription information for this workspace
    pub subscription: Option<WorkspaceSubscription>,
    /// Expected working hours per day in minutes (for capacity planning)
    pub working_hours_in_minutes: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_data_retention_days: Option<i32>,
}

/// Information about CSV data uploads to a workspace.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CSVUpload {
    /// Timestamp of the last CSV upload
    pub at: Option<DateTime<Utc>>,
}

/// Subscription details for a workspace.
///
/// Contains billing, payment, and plan information for paid workspaces.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceSubscription {
    pub auto_renew: bool,
    pub card_details: Option<CardDetails>,
    pub company_id: Option<CompanyId>,
    pub contact_detail: Option<ContactDetail>,
    pub created_at: DateTime<Utc>,
    pub currency: String,
    pub customer_id: Option<CustomerId>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub last_pricing_plan_id: Option<PricingPlanId>,
    pub organization_id: Option<OrganizationId>,
    pub payment_details: Option<PaymentDetails>,
    pub pricing_plan_id: PricingPlanId,
    pub renewal_at: Option<DateTime<Utc>>,
    pub subscription_id: Option<SubscriptionId>,
    pub subscription_period: Option<SubscriptionPeriod>,
    pub workspace_id: WorkspaceId,
}

/// Credit card details for workspace billing.
///
/// **Note**: Card numbers are masked for security.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardDetails {
    pub added_at: DateTime<Utc>,
    pub card_number: String,
    pub card_type: String,
    pub creator_id: UserId,
    pub creator_name: String,
    pub expiry_date: String,
    pub holder_name: String,
}

/// Billing contact information for a workspace.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContactDetail {
    pub company_name: Option<String>,
    pub company_address: Option<String>,
    pub company_city: Option<String>,
    pub company_state: Option<String>,
    pub company_zip: Option<String>,
    pub company_country_id: Option<CountryId>,
    pub contact_person: Option<String>,
    pub contact_email: Option<String>,
}

/// Payment information for workspace subscription.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentDetails {
    pub created_at: DateTime<Utc>,
    pub currency: String,
    pub next_payment_date: Option<DateTime<Utc>>,
    pub payment_type: String,
}

/// Information about the current subscription period.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionPeriod {
    pub current_period_ends_at: Option<DateTime<Utc>>,
    pub current_period_starts_at: Option<DateTime<Utc>>,
    pub last_pricing_plan_id: Option<PricingPlanId>,
    pub pricing_plan_id: PricingPlanId,
    pub trial: bool,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub trial_extended_at: Option<DateTime<Utc>>,
}

/// Request structure for creating a new workspace.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
/// use toggl_api::models::api::workspace::CreateWorkspace;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let new_workspace = CreateWorkspace {
///     name: "My New Workspace".to_string(),
///     initial_pricing_plan: None, // Use default free plan
/// };
/// let workspace = client.create_workspace(new_workspace).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateWorkspace {
    pub name: String,
    pub initial_pricing_plan: Option<PricingPlanId>,
}

/// Request structure for updating workspace settings.
///
/// All fields are optional - only include the fields you want to update.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
/// use toggl_api::models::api::workspace::UpdateWorkspace;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// let update = UpdateWorkspace {
///     name: Some("Updated Workspace Name".to_string()),
///     default_currency: Some("EUR".to_string()),
///     rounding: Some(1), // Round up
///     rounding_minutes: Some(15), // Round to 15 minutes
///     ..Default::default()
/// };
///
/// let updated = client.update_workspace(workspace_id, update).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateWorkspace {
    pub name: Option<String>,
    pub default_hourly_rate: Option<f64>,
    pub default_currency: Option<String>,
    pub only_admins_may_create_projects: Option<bool>,
    pub only_admins_may_create_tags: Option<bool>,
    pub only_admins_see_billable_rates: Option<bool>,
    pub only_admins_see_team_dashboard: Option<bool>,
    pub projects_billable_by_default: Option<bool>,
    pub projects_enforce_billable: Option<bool>,
    pub projects_private_by_default: Option<bool>,
    pub rate_change_mode: Option<String>,
    pub reports_collapse: Option<bool>,
    pub rounding: Option<i32>,
    pub rounding_minutes: Option<i32>,
}

/// Represents a user's membership in a workspace.
///
/// Contains information about the user's role, permissions, and rates within
/// a specific workspace.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceUser {
    /// Unique identifier for this workspace-user relationship
    pub id: WorkspaceUserId,
    /// User's global ID
    pub user_id: UserId,
    /// Workspace this membership is for
    pub workspace_id: WorkspaceId,
    /// User's display name
    pub name: String,
    /// User's email address
    pub email: String,
    /// Whether user is an admin in this workspace
    pub admin: bool,
    /// Whether the user is active in this workspace
    pub active: bool,
    /// Invitation URL if user hasn't accepted yet
    pub invite_url: Option<String>,
    /// Server timestamp when this data was retrieved
    pub at: DateTime<Utc>,
    /// Filename of user's avatar image
    pub avatar_file_name: Option<String>,
    /// Groups this user belongs to in the workspace
    pub group_ids: Option<Vec<GroupId>>,
    /// Whether the user is inactive (inverse of active)
    pub inactive: bool,
    /// Whether this is a direct workspace member (not via group)
    pub is_direct: bool,
    /// User's labor cost per hour (for profitability reports)
    pub labour_cost: Option<f64>,
    /// User's billable rate per hour
    pub rate: Option<f64>,
    /// When the user's rate was last updated
    pub rate_last_updated: Option<DateTime<Utc>>,
    /// User's role in the workspace (e.g., "admin", "user")
    pub role: String,
    /// User's timezone
    pub timezone: String,
    /// Expected working hours per day in minutes
    pub working_hours_in_minutes: Option<u32>,
}

/// Statistics about a workspace's users and groups.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceStatistics {
    /// Number of admin users in the workspace
    pub admins_count: u32,
    /// Number of groups in the workspace
    pub groups_count: u32,
    /// Total number of members (active + inactive)
    pub members_count: u32,
    /// Number of active members
    pub active_members_count: u32,
    /// Number of pending invitations
    pub invited_members_count: u32,
}
