use serde::{Deserialize, Serialize};

use toggl_core::{AlertId, WorkspaceId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: AlertId,
    pub workspace_id: WorkspaceId,
    pub alert_type: String,
    pub threshold: f64,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertWithMeta {
    #[serde(flatten)]
    pub alert: Alert,
    pub meta: AlertMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertMeta {
    pub current_value: f64,
    pub triggered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlert {
    pub alert_type: String,
    pub threshold: f64,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAlert {
    pub threshold: Option<f64>,
    pub enabled: Option<bool>,
}
