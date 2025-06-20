use super::ids::{CreatorId, GoalId, ProjectId, TagId, TaskId, UserId, WorkspaceId};
use serde::{Deserialize, Serialize};

/// Workspace goal response from API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceGoal {
    #[serde(rename = "goal_id")]
    pub id: GoalId,
    pub active: bool,
    pub billable: bool,
    pub comparison: String,
    pub creator_user_id: CreatorId,
    pub creator_user_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_recurrence_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_recurrence_start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_recurrence_tracked_seconds: Option<i64>,
    pub end_date: String,
    pub icon: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_completed_recurrence_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_notified_at: Option<String>,
    pub name: String,
    pub permissions: Vec<String>,
    pub project_ids: Vec<ProjectId>,
    pub recurrence: String,
    pub start_date: String,
    pub status: String,
    pub streak: i32,
    pub tag_ids: Vec<TagId>,
    pub tags: Vec<String>,
    pub target_seconds: i64,
    pub task_ids: Vec<TaskId>,
    pub team_goal: bool,
    pub user_id: UserId,
    pub user_name: String,
    pub workspace_id: WorkspaceId,
}

/// Goal model for create/update responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: GoalId,
    pub active: bool,
    pub billable: bool,
    pub comparison: String,
    #[serde(rename = "creatorUserID")]
    pub creator_user_id: CreatorId,
    #[serde(rename = "creatorUserName")]
    pub creator_user_name: String,
    #[serde(
        rename = "currentRecurrenceEndDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_recurrence_end_date: Option<String>,
    #[serde(
        rename = "currentRecurrenceStartDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_recurrence_start_date: Option<String>,
    #[serde(
        rename = "currentRecurrenceTrackedSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_recurrence_tracked_seconds: Option<i64>,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub icon: String,
    #[serde(
        rename = "lastCompletedRecurrenceEndDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_completed_recurrence_end_date: Option<String>,
    #[serde(rename = "lastNotifiedAt", skip_serializing_if = "Option::is_none")]
    pub last_notified_at: Option<String>,
    pub name: String,
    pub permissions: Vec<String>,
    #[serde(rename = "projectIDs")]
    pub project_ids: Vec<ProjectId>,
    pub recurrence: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    pub status: String,
    pub streak: i32,
    #[serde(rename = "tagIDs")]
    pub tag_ids: Vec<TagId>,
    pub tags: Vec<String>,
    #[serde(rename = "targetSeconds")]
    pub target_seconds: i64,
    #[serde(rename = "taskIDs")]
    pub task_ids: Vec<TaskId>,
    #[serde(rename = "teamGoal")]
    pub team_goal: bool,
    #[serde(rename = "userID")]
    pub user_id: UserId,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "workspaceID")]
    pub workspace_id: WorkspaceId,
}

/// Create goal request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGoalRequest {
    pub billable: bool,
    pub comparison: String,
    pub end_date: String,
    pub icon: String,
    pub name: String,
    pub project_ids: Vec<ProjectId>,
    pub recurrence: String,
    pub start_date: String,
    pub tag_ids: Vec<TagId>,
    pub target_seconds: i64,
    pub task_ids: Vec<TaskId>,
    pub user_id: UserId,
}

/// Update goal request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGoalRequest {
    pub active: bool,
    pub comparison: String,
    pub end_date: String,
    pub icon: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_notified_at: Option<String>,
    pub name: String,
    pub target_seconds: i64,
}

/// Query parameters for listing workspace goals
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceGoalsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_goals: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}
