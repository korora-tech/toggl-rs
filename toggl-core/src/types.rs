//! Common types used across Toggl APIs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Common ID type used across Toggl APIs
pub type Id = i64;

/// Workspace ID type
pub type WorkspaceId = Id;

/// User ID type
pub type UserId = Id;

/// Project ID type
pub type ProjectId = Id;

/// Client ID type
pub type ClientId = Id;

/// Tag ID type
pub type TagId = Id;

/// Time entry ID type
pub type TimeEntryId = Id;

/// Common timestamp type
pub type Timestamp = DateTime<Utc>;

/// Common response wrapper used by some Toggl endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWrapper<T> {
    pub data: T,
}

/// Pagination parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

/// Sort order
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}
