use serde::{Deserialize, Serialize};

use toggl_core::{GoalId, ProjectId, WorkspaceId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: GoalId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub duration: u64,
    pub project_ids: Vec<ProjectId>,
    pub active: bool,
}
