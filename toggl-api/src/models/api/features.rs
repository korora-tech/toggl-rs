use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feature {
    pub feature_id: u32,
    pub name: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Features {
    pub workspace_id: u64,
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
    pub group_ids: Option<Vec<u64>>,
    pub reminder_id: u64,
    pub threshold: u32,
    pub user_ids: Option<Vec<u64>>,
    pub workspace_id: u64,
}
