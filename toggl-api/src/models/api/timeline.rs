use serde::{Deserialize, Serialize};

use toggl_core::{ProjectId, TaskId, TimelineId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub id: TimelineId,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration: u64,
    pub description: Option<String>,
    pub activity_type: String,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimelineEvent {
    pub start_time: String,
    pub end_time: Option<String>,
    pub description: Option<String>,
    pub activity_type: String,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
}
