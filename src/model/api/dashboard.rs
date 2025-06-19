use super::{ProjectId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub user_id: UserId,
    pub project_id: Option<ProjectId>,
    pub duration: i64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllActivity {
    pub activity: Vec<Activity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MostActiveUser {
    pub user_id: UserId,
    pub duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MostActive {
    pub most_active: Vec<MostActiveUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopActivity {
    pub project_id: Option<ProjectId>,
    pub duration: i64,
    pub user_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopActivities {
    pub top_activity: Vec<TopActivity>,
}
