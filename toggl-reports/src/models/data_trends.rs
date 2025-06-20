//! Data trends report models

use serde::{Deserialize, Serialize};
use toggl_core::{ClientId, ProjectId, UserId};

/// Data trends response for clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientDataTrendsReport {
    pub data: Vec<ClientDataTrend>,
}

/// Client data trend entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientDataTrend {
    /// Client ID
    pub client_id: Option<ClientId>,

    /// Client name
    pub client_name: Option<String>,

    /// Graph data points
    pub graph: Vec<DataTrendsGraphData>,

    /// Total time
    pub total_seconds: i64,
}

/// Data trends response for projects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDataTrendsReport {
    pub data: Vec<ProjectDataTrend>,
}

/// Project data trend entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDataTrend {
    /// Project ID
    pub project_id: ProjectId,

    /// Project name
    pub project_name: String,

    /// Client ID
    pub client_id: Option<ClientId>,

    /// Client name
    pub client_name: Option<String>,

    /// Graph data points
    pub graph: Vec<DataTrendsGraphData>,

    /// Total time
    pub total_seconds: i64,
}

/// Data trends response for users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataTrendsReport {
    pub data: Vec<UserDataTrend>,
}

/// User data trend entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataTrend {
    /// User ID
    pub user_id: UserId,

    /// User name
    pub user_name: String,

    /// Graph data points
    pub graph: Vec<DataTrendsGraphData>,

    /// Total time
    pub total_seconds: i64,
}

/// Data trends graph data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTrendsGraphData {
    /// Date
    pub date: String,

    /// Time in seconds
    pub seconds: i64,
}

/// Data trends graph wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTrendsGraph {
    pub graph: Vec<DataTrendsGraphData>,
}
