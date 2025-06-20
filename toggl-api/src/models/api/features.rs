use super::ids::{GroupId, ReminderId, UserId, WorkspaceId};
use serde::{Deserialize, Serialize};

use super::ids::FeatureId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feature {
    pub feature_id: FeatureId,
    pub name: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Features {
    pub workspace_id: WorkspaceId,
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub city: String,
    pub city_lat_long: String,
    pub state: String,
    pub country_code: String,
    pub country_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackReminder {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub frequency: u32,
    pub group_ids: Option<Vec<GroupId>>,
    pub reminder_id: ReminderId,
    pub threshold: u32,
    pub user_ids: Option<Vec<UserId>>,
    pub workspace_id: WorkspaceId,
}
