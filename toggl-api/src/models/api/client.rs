use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    pub id: u64,
    pub workspace_id: u64,
    pub name: String,
    pub archived: bool,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
    pub permissions: Option<Vec<String>>,

    #[deprecated(note = "Use workspace_id instead")]
    pub wid: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateClient {
    pub workspace_id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateClient {
    pub name: Option<String>,
    pub archived: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulkArchiveClients {
    pub client_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArchiveClientsResponse {
    pub client_ids: Option<Vec<u64>>,
    pub project_ids: Option<Vec<u64>>,
}
