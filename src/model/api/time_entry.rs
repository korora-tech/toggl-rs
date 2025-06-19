use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntry {
    pub id: u64,
    pub workspace_id: u64,
    pub user_id: u64,
    pub billable: bool,
    pub start: DateTime<Utc>,
    pub stop: Option<DateTime<Utc>>,
    pub duration: i64,
    pub description: Option<String>,
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
    pub tag_ids: Option<Vec<u64>>,
    pub tags: Option<Vec<String>>,
    pub at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duronly: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_with: Option<Vec<TimeEntrySharedWith>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_ids: Option<Vec<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_billable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_avatar_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use workspace_id instead")]
    pub wid: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use project_id instead")]
    pub pid: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use task_id instead")]
    pub tid: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use user_id instead")]
    pub uid: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntrySharedWith {
    pub accepted: bool,
    pub user_id: u64,
    pub user_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTimeEntry {
    pub workspace_id: u64,
    pub start: DateTime<Utc>,
    pub duration: i64,
    pub description: Option<String>,
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
    pub tag_ids: Option<Vec<u64>>,
    pub billable: Option<bool>,
    pub created_with: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTimeEntry {
    pub description: Option<String>,
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
    pub tag_ids: Option<Vec<u64>>,
    pub billable: Option<bool>,
    pub start: Option<DateTime<Utc>>,
    pub stop: Option<DateTime<Utc>>,
    pub duration: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulkEditTimeEntries {
    pub time_entry_ids: Vec<u64>,
    pub op: BulkEditOperation,
    pub path: String,
    pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BulkEditOperation {
    Add,
    Remove,
    Replace,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntryChecklist {
    pub errors: Vec<TimeEntryChecklistError>,
    pub warnings: Vec<TimeEntryChecklistWarning>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntryChecklistError {
    pub code: String,
    pub message: String,
    pub time_entry_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntryChecklistWarning {
    pub code: String,
    pub message: String,
    pub time_entry_ids: Vec<u64>,
}
