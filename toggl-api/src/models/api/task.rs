use super::ids::{ProjectId, TaskId, UserId, WorkspaceId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub project_id: ProjectId,
    pub workspace_id: WorkspaceId,
    pub user_id: Option<UserId>,
    pub estimated_seconds: Option<u64>,
    pub tracked_seconds: Option<u64>,
    pub active: bool,
    pub recurring: bool,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTask {
    pub workspace_id: WorkspaceId,
    pub project_id: ProjectId,
    pub name: String,
    pub estimated_seconds: Option<u64>,
    pub active: Option<bool>,
    pub user_id: Option<UserId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTask {
    pub name: Option<String>,
    pub estimated_seconds: Option<u64>,
    pub active: Option<bool>,
    pub user_id: Option<UserId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulkDeleteTasks {
    pub task_ids: Vec<TaskId>,
}
