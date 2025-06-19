use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: u64,
    pub workspace_id: u64,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub duration: u64,
    pub project_ids: Vec<u64>,
    pub active: bool,
}
