use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use toggl_core::{
    CreatorId, LevelId, ProjectId, ProjectUserId, RateId, TaskId, WorkspaceId, WorkspaceUserId,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rate {
    pub id: Option<RateId>,
    pub amount: Option<f64>,
    pub created_at: Option<DateTime<Utc>>,
    pub creator_id: Option<CreatorId>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub start: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub workspace_id: Option<WorkspaceId>,
    pub project_id: Option<ProjectId>,
    pub planned_task_id: Option<TaskId>,
    pub workspace_user_id: Option<WorkspaceUserId>,
    pub project_user_id: Option<ProjectUserId>,
    #[serde(rename = "type")]
    pub rate_type: Option<String>,
    pub rate_change_mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateRate {
    pub amount: f64,
    pub level: RateLevel,
    pub level_id: LevelId,
    #[serde(rename = "type")]
    pub rate_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<RateMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RateLevel {
    Workspace,
    WorkspaceUser,
    Project,
    ProjectUser,
    Task,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum RateMode {
    OverrideAll,
    OverrideCurrent,
    StartToday,
}

impl std::fmt::Display for RateLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RateLevel::Workspace => write!(f, "workspace"),
            RateLevel::WorkspaceUser => write!(f, "workspace_user"),
            RateLevel::Project => write!(f, "project"),
            RateLevel::ProjectUser => write!(f, "project_user"),
            RateLevel::Task => write!(f, "task"),
        }
    }
}
