use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSegmentation {
    pub demo_requested: Option<bool>,
    pub full_name: Option<String>,
    pub heard: Option<Vec<String>>,
    pub industries: Option<Vec<String>>,
    pub members_range: Option<String>,
    pub organization_id: Option<u64>,
    pub reasons: Option<Vec<String>>,
    pub skipped_step: Option<String>,
    pub user_id: Option<u64>,
    pub user_segments: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceAssignment {
    pub group_id: u64,
    pub user_id: u64,
    pub workspace_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceAssignment {
    pub user_id: u64,
    pub group_id: u64,
}
