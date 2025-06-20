//! Search models for reports API

use serde::{Deserialize, Serialize};

/// Search clients response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchClientsResponse {
    pub data: Vec<ClientSearchResult>,
}

/// Client search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientSearchResult {
    pub id: i64,
    pub name: String,
    pub archived: bool,
}

/// Search projects response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchProjectsResponse {
    pub data: Vec<ProjectSearchResult>,
}

/// Project search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSearchResult {
    pub id: i64,
    pub name: String,
    pub client_id: Option<i64>,
    pub active: bool,
    pub billable: bool,
    pub color: String,
}

/// Search users response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsersResponse {
    pub data: Vec<UserSearchResult>,
}

/// User search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchResult {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub active: bool,
}

/// Search parameters for various searches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    /// Search query
    pub q: String,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}
