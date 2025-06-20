use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use toggl_core::{
    ClientId, GroupId, ProjectGroupId, ProjectId, ProjectUserId, UserId, WorkspaceId,
};

/// Represents a Toggl project.
///
/// Projects are used to organize time entries and can be associated with clients
/// for billing purposes. They support features like budgets, templates, and recurring periods.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// // Get all active projects
/// let projects = client.get_projects(workspace_id, Some(true), None, None).await?;
///
/// for project in projects {
///     println!("Project: {} ({})", project.name, project.color);
///     println!("Billable: {:?}", project.billable);
///     if let Some(client_id) = project.client_id {
///         println!("Client ID: {}", client_id);
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    /// Unique identifier for the project
    pub id: ProjectId,
    /// Workspace this project belongs to
    pub workspace_id: WorkspaceId,
    /// Associated client ID (for client billing)
    pub client_id: Option<ClientId>,
    /// Project name
    pub name: String,
    /// Whether this project is private (visible only to assigned users)
    pub is_private: bool,
    /// Whether the project is active
    pub active: bool,
    /// Server timestamp when this data was retrieved
    pub at: DateTime<Utc>,
    /// When the project was created
    pub created_at: DateTime<Utc>,
    /// When the project was deleted (if applicable)
    pub server_deleted_at: Option<DateTime<Utc>>,
    /// Project color in hex format (e.g., "#06aaf5")
    pub color: String,
    /// Whether time entries for this project are billable by default
    pub billable: Option<bool>,
    /// Whether this is a template project
    pub template: Option<bool>,
    /// Whether to use automatic time estimates
    pub auto_estimates: Option<bool>,
    /// Estimated hours for the project
    pub estimated_hours: Option<u32>,
    /// Estimated seconds for the project (more precise than hours)
    pub estimated_seconds: Option<u64>,
    /// Hourly rate for this project
    pub rate: Option<f64>,
    /// When the rate was last updated
    pub rate_last_updated: Option<DateTime<Utc>>,
    /// Currency code for the rate (ISO 4217)
    pub currency: Option<String>,
    /// Whether this is a recurring project
    pub recurring: bool,
    /// Parameters for recurring projects
    pub recurring_parameters: Option<Vec<RecurringParameter>>,
    /// Current billing period for recurring projects
    pub current_period: Option<CurrentPeriod>,
    /// Fixed fee for the entire project
    pub fixed_fee: Option<f64>,
    /// Actual hours tracked (calculated field)
    pub actual_hours: Option<u32>,
    /// Actual seconds tracked (more precise than hours)
    pub actual_seconds: Option<u64>,
    /// Project start date
    pub start_date: Option<NaiveDate>,
    /// Project end date
    pub end_date: Option<NaiveDate>,
    /// When the first time entry was created for this project
    pub first_time_entry: Option<DateTime<Utc>>,
    /// User's permissions for this project
    pub permissions: Option<Vec<String>>,

    #[deprecated(note = "Use workspace_id instead")]
    pub wid: Option<WorkspaceId>,

    #[deprecated(note = "Use client_id instead")]
    pub cid: Option<ClientId>,
}

/// Parameters for recurring project periods.
///
/// Used to define how a project's budget and timeline repeat over time.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecurringParameter {
    pub custom_period: Option<i64>,
    pub estimated_seconds: i64,
    pub parameter_end_date: Option<NaiveDate>,
    pub parameter_start_date: NaiveDate,
    pub period: String,
    pub project_start_date: NaiveDate,
}

/// Current billing period for a recurring project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentPeriod {
    pub end_date: NaiveDate,
    pub start_date: NaiveDate,
}

/// Request structure for creating a new project.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
/// use toggl_api::models::api::project::CreateProject;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// let new_project = CreateProject {
///     workspace_id,
///     name: "Website Redesign".to_string(),
///     client_id: Some(ClientId(789)),
///     is_private: Some(false),
///     active: Some(true),
///     color: Some("#06aaf5".to_string()),
///     billable: Some(true),
///     rate: Some(100.0),
///     currency: Some("USD".to_string()),
///     template: None,
///     auto_estimates: None,
///     estimated_hours: None,
///     recurring: None,
///     recurring_parameters: None,
///     fixed_fee: None,
///     start_date: None,
///     end_date: None
/// };
///
/// let project = client.create_project(workspace_id, new_project).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProject {
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub client_id: Option<ClientId>,
    pub is_private: Option<bool>,
    pub active: Option<bool>,
    pub color: Option<String>,
    pub billable: Option<bool>,
    pub template: Option<bool>,
    pub auto_estimates: Option<bool>,
    pub estimated_hours: Option<u32>,
    pub rate: Option<f64>,
    pub currency: Option<String>,
    pub recurring: Option<bool>,
    pub recurring_parameters: Option<Vec<RecurringParameter>>,
    pub fixed_fee: Option<f64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

/// Request structure for updating an existing project.
///
/// All fields are optional - only include the fields you want to update.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub client_id: Option<ClientId>,
    pub is_private: Option<bool>,
    pub active: Option<bool>,
    pub color: Option<String>,
    pub billable: Option<bool>,
    pub template: Option<bool>,
    pub auto_estimates: Option<bool>,
    pub estimated_hours: Option<u32>,
    pub rate: Option<f64>,
    pub currency: Option<String>,
    pub recurring: Option<bool>,
    pub recurring_parameters: Option<Vec<RecurringParameter>>,
    pub fixed_fee: Option<f64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

/// Represents a user's assignment to a project.
///
/// Contains user-specific settings like custom rates and manager permissions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectUser {
    pub id: ProjectUserId,
    pub project_id: ProjectId,
    pub user_id: UserId,
    pub workspace_id: WorkspaceId,
    pub manager: bool,
    pub rate: Option<f64>,
    pub rate_last_updated: Option<DateTime<Utc>>,
    pub labour_cost: Option<f64>,
    pub at: DateTime<Utc>,
    pub group_ids: Option<Vec<GroupId>>,
}

/// Represents a group's assignment to a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectGroup {
    pub id: ProjectGroupId,
    pub group_id: GroupId,
    pub project_id: ProjectId,
    pub workspace_id: WorkspaceId,
}

/// Statistics for a project.
///
/// Shows estimated vs actual time tracked and billable hours.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectStatistics {
    pub estimated_seconds: Option<u64>,
    pub tracked_seconds: Option<u64>,
    pub billable_seconds: Option<u64>,
}

/// Represents a time period for a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectPeriod {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

/// JSON Patch operation for bulk updates.
///
/// Used for bulk operations on projects following RFC 6902.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatchOperation {
    pub op: String,
    pub path: String,
    pub value: Option<serde_json::Value>,
}

/// Container for multiple project IDs.
///
/// Used in bulk operations that affect multiple projects.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectIds {
    pub project_ids: Vec<ProjectId>,
}

/// Request structure for adding a user to a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProjectUser {
    pub project_id: ProjectId,
    pub user_id: UserId,
    pub manager: Option<bool>,
    pub rate: Option<f64>,
    pub labour_cost: Option<f64>,
}

/// Request structure for updating a user's project assignment.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProjectUser {
    pub manager: Option<bool>,
    pub rate: Option<f64>,
    pub labour_cost: Option<f64>,
}

/// Request structure for assigning a group to a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectGroupPayload {
    pub project_id: ProjectId,
    pub group_id: GroupId,
}

/// Represents a project template.
///
/// Templates can be used to quickly create new projects with predefined settings.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectTemplate {
    pub id: ProjectId,
    pub name: String,
    pub active: bool,
    pub billable: Option<bool>,
    pub is_private: bool,
    pub color: String,
    pub auto_estimates: Option<bool>,
    pub estimated_hours: Option<u32>,
    pub rate: Option<f64>,
}
