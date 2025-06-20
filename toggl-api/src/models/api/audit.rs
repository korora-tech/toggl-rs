use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use toggl_core::{AuditId, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: AuditId,
    pub timestamp: DateTime<Utc>,
    pub user_id: UserId,
    pub event_type: String,
    pub details: serde_json::Value,
}
