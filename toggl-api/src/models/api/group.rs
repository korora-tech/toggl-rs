use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: u64,
    pub workspace_id: u64,
    pub organization_id: Option<u64>,
    pub name: String,
    pub at: DateTime<Utc>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGroup {
    pub workspace_id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateGroup {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupUser {
    pub id: u64,
    pub group_id: u64,
    pub user_id: u64,
    pub workspace_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatchGroupUsersInput {
    pub add: Option<Vec<u64>>,
    pub remove: Option<Vec<u64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatchGroupUsersOutput {
    pub success: Vec<u64>,
    pub failure: Vec<PatchFailure>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatchFailure {
    pub id: u64,
    pub message: String,
}
