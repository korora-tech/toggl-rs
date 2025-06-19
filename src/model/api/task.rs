use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub project_id: u64,
    pub workspace_id: u64,
    pub user_id: Option<u64>,
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
    pub workspace_id: u64,
    pub project_id: u64,
    pub name: String,
    pub estimated_seconds: Option<u64>,
    pub active: Option<bool>,
    pub user_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTask {
    pub name: Option<String>,
    pub estimated_seconds: Option<u64>,
    pub active: Option<bool>,
    pub user_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulkDeleteTasks {
    pub task_ids: Vec<u64>,
}
