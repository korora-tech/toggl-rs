use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Favorite {
    pub id: u64,
    pub workspace_id: u64,
    pub user_id: u64,
    pub description: Option<String>,
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
    pub tag_ids: Option<Vec<u64>>,
    pub billable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub suggestion: bool,
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateFavorite {
    pub workspace_id: u64,
    pub description: Option<String>,
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
    pub tag_ids: Option<Vec<u64>>,
    pub billable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFavorite {
    pub description: Option<String>,
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
    pub tag_ids: Option<Vec<u64>>,
    pub billable: Option<bool>,
}
