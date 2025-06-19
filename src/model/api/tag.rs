use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub id: u64,
    pub workspace_id: u64,
    pub name: String,
    pub at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTag {
    pub workspace_id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTag {
    pub name: String,
}
