use crate::models::api::ids::{FavoriteId, ProjectId, TagId, TaskId, UserId, WorkspaceId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Favorite {
    pub id: FavoriteId,
    pub workspace_id: WorkspaceId,
    pub user_id: UserId,
    pub description: Option<String>,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub tag_ids: Option<Vec<TagId>>,
    pub billable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub suggestion: bool,
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateFavorite {
    pub workspace_id: WorkspaceId,
    pub description: Option<String>,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub tag_ids: Option<Vec<TagId>>,
    pub billable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFavorite {
    pub description: Option<String>,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub tag_ids: Option<Vec<TagId>>,
    pub billable: Option<bool>,
}
