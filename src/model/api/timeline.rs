use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub id: u64,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration: u64,
    pub description: Option<String>,
    pub activity_type: String,
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimelineEvent {
    pub start_time: String,
    pub end_time: Option<String>,
    pub description: Option<String>,
    pub activity_type: String,
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
}
