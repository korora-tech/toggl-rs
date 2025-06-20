use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::ids::{TagId, WorkspaceId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub id: TagId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTag {
    pub workspace_id: WorkspaceId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTag {
    pub name: String,
}
