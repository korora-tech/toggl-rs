//! Dictionary models for shared reference data

use serde::{Deserialize, Serialize};
use toggl_core::{ClientId, GroupId, ProjectId, ProjectUserId, TagId, TaskId, UserId};

/// Report dictionaries containing reference data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDictionaries {
    pub clients: Option<Vec<ClientDict>>,
    pub projects: Option<Vec<ProjectDict>>,
    pub users: Option<Vec<UserDict>>,
    pub tasks: Option<Vec<TaskDict>>,
    pub tags: Option<Vec<TagDict>>,
    pub groups: Option<Vec<GroupDict>>,
}

/// Client dictionary entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientDict {
    pub id: ClientId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

/// Project dictionary entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDict {
    pub id: ProjectId,
    pub name: String,
    pub client_id: Option<ClientId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable: Option<bool>,
}

/// User dictionary entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDict {
    pub id: UserId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

/// Task dictionary entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDict {
    pub id: TaskId,
    pub name: String,
    pub project_id: ProjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// Tag dictionary entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDict {
    pub id: TagId,
    pub name: String,
}

/// Group dictionary entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDict {
    pub id: GroupId,
    pub name: String,
}

/// Report dictionary data wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDictionariesData {
    pub data: ReportDictionaries,
}

/// General dictionary wrapper for various report types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralDictionary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<ClientDictionary>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<ProjectDictionary>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<UserDictionary>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<TaskDictionary>,
}

/// Client dictionary wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientDictionary {
    #[serde(flatten)]
    pub clients: std::collections::HashMap<String, ClientDict>,
}

/// Project dictionary wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDictionary {
    #[serde(flatten)]
    pub projects: std::collections::HashMap<String, ProjectDict>,
}

/// User dictionary wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDictionary {
    #[serde(flatten)]
    pub users: std::collections::HashMap<String, UserDict>,
}

/// Task dictionary wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDictionary {
    #[serde(flatten)]
    pub tasks: std::collections::HashMap<String, TaskDict>,
}

/// Project user dictionary entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUserDict {
    pub id: ProjectUserId,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}
