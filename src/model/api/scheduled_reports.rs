use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledReport {
    pub bookmark_id: Option<i64>,
    pub created_at: Option<String>,
    pub creator_id: Option<i64>,
    pub deleted_at: Option<String>,
    pub frequency: Option<i32>,
    pub group_ids: Option<Vec<i64>>,
    pub report_id: Option<i64>,
    pub user_ids: Option<Vec<i64>>,
    pub workspace_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScheduledReportPayload {
    pub bookmark_id: i64,
    pub frequency: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<i64>>,
}
