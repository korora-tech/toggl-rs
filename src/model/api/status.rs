use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStatus {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}
