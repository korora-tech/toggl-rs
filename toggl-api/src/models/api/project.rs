use super::ids::{
    ClientId, GroupId, ProjectGroupId, ProjectId, ProjectUserId, UserId, WorkspaceId,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub id: ProjectId,
    pub workspace_id: WorkspaceId,
    pub client_id: Option<ClientId>,
    pub name: String,
    pub is_private: bool,
    pub active: bool,
    pub at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
    pub color: String,
    pub billable: Option<bool>,
    pub template: Option<bool>,
    pub auto_estimates: Option<bool>,
    pub estimated_hours: Option<u32>,
    pub estimated_seconds: Option<u64>,
    pub rate: Option<f64>,
    pub rate_last_updated: Option<DateTime<Utc>>,
    pub currency: Option<String>,
    pub recurring: bool,
    pub recurring_parameters: Option<Vec<RecurringParameter>>,
    pub current_period: Option<CurrentPeriod>,
    pub fixed_fee: Option<f64>,
    pub actual_hours: Option<u32>,
    pub actual_seconds: Option<u64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub first_time_entry: Option<DateTime<Utc>>,
    pub permissions: Option<Vec<String>>,

    #[deprecated(note = "Use workspace_id instead")]
    pub wid: Option<WorkspaceId>,

    #[deprecated(note = "Use client_id instead")]
    pub cid: Option<ClientId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecurringParameter {
    pub custom_period: Option<i64>,
    pub estimated_seconds: i64,
    pub parameter_end_date: Option<NaiveDate>,
    pub parameter_start_date: NaiveDate,
    pub period: String,
    pub project_start_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentPeriod {
    pub end_date: NaiveDate,
    pub start_date: NaiveDate,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectGroup {
    pub id: ProjectGroupId,
    pub group_id: GroupId,
    pub project_id: ProjectId,
    pub workspace_id: WorkspaceId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectStatistics {
    pub estimated_seconds: Option<u64>,
    pub tracked_seconds: Option<u64>,
    pub billable_seconds: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectPeriod {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatchOperation {
    pub op: String,
    pub path: String,
    pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectIds {
    pub project_ids: Vec<ProjectId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProjectUser {
    pub project_id: ProjectId,
    pub user_id: UserId,
    pub manager: Option<bool>,
    pub rate: Option<f64>,
    pub labour_cost: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProjectUser {
    pub manager: Option<bool>,
    pub rate: Option<f64>,
    pub labour_cost: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectGroupPayload {
    pub project_id: ProjectId,
    pub group_id: GroupId,
}

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
