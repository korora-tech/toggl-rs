use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use toggl_core::{
    CompanyId, CountryId, CustomerId, GroupId, OrganizationId, PricingPlanId, SubscriptionId,
    UserId, WorkspaceId, WorkspaceUserId,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub organization_id: Option<OrganizationId>,
    pub name: String,
    pub profile: u64,
    pub premium: bool,
    pub business_ws: bool,
    pub admin: bool,
    pub default_hourly_rate: Option<f64>,
    pub default_currency: String,
    pub only_admins_may_create_projects: bool,
    pub only_admins_may_create_tags: bool,
    pub only_admins_see_billable_rates: bool,
    pub only_admins_see_team_dashboard: bool,
    pub projects_billable_by_default: bool,
    pub projects_enforce_billable: bool,
    pub projects_private_by_default: bool,
    pub rate_last_updated: Option<DateTime<Utc>>,
    pub reports_collapse: bool,
    pub rounding: i32,
    pub rounding_minutes: i32,
    pub api_token: Option<String>,
    pub at: DateTime<Utc>,
    pub ical_enabled: bool,
    pub ical_url: Option<String>,
    pub csv_upload: Option<CSVUpload>,
    pub subscription: Option<WorkspaceSubscription>,
    pub working_hours_in_minutes: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_data_retention_days: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CSVUpload {
    pub at: Option<DateTime<Utc>>,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentDetails {
    pub created_at: DateTime<Utc>,
    pub currency: String,
    pub next_payment_date: Option<DateTime<Utc>>,
    pub payment_type: String,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateWorkspace {
    pub name: String,
    pub initial_pricing_plan: Option<PricingPlanId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceUser {
    pub id: WorkspaceUserId,
    pub user_id: UserId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub active: bool,
    pub invite_url: Option<String>,
    pub at: DateTime<Utc>,
    pub avatar_file_name: Option<String>,
    pub group_ids: Option<Vec<GroupId>>,
    pub inactive: bool,
    pub is_direct: bool,
    pub labour_cost: Option<f64>,
    pub rate: Option<f64>,
    pub rate_last_updated: Option<DateTime<Utc>>,
    pub role: String,
    pub timezone: String,
    pub working_hours_in_minutes: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceStatistics {
    pub admins_count: u32,
    pub groups_count: u32,
    pub members_count: u32,
    pub active_members_count: u32,
    pub invited_members_count: u32,
}
