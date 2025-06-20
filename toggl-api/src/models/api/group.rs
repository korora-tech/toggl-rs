use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use toggl_core::{GroupId, OrganizationId, UserId, WorkspaceId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: GroupId,
    pub workspace_id: WorkspaceId,
    pub organization_id: Option<OrganizationId>,
    pub name: String,
    pub at: DateTime<Utc>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGroup {
    pub workspace_id: WorkspaceId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateGroup {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupUser {
    pub id: u64,
    pub group_id: GroupId,
    pub user_id: UserId,
    pub workspace_id: WorkspaceId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatchGroupUsersInput {
    pub add: Option<Vec<UserId>>,
    pub remove: Option<Vec<UserId>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatchGroupUsersOutput {
    pub success: Vec<UserId>,
    pub failure: Vec<PatchFailure>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatchFailure {
    pub id: UserId,
    pub message: String,
}
