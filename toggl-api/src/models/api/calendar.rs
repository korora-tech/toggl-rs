use crate::models::api::ids::{IntegrationId, ProjectId, TaskId, WorkspaceId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::ids::{CalendarId, ProviderUserId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Integration {
    pub id: IntegrationId,
    pub provider: String,
    pub provider_user_id: ProviderUserId,
    pub linked_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Calendar {
    pub id: CalendarId,
    pub integration_id: IntegrationId,
    pub name: String,
    pub description: Option<String>,
    pub selected: bool,
    pub color: Option<String>,
    pub ical_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarEvent {
    pub id: CalendarId,
    pub calendar_id: CalendarId,
    pub integration_id: IntegrationId,
    pub title: String,
    pub description: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub all_day: bool,
    pub recurring: bool,
    pub color: Option<String>,
    pub location: Option<String>,
    pub attendees: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarsResponse {
    pub calendars: Vec<Calendar>,
    pub next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventsResponse {
    pub events: Vec<CalendarEvent>,
    pub next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventDetailsSuggestion {
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub billable: Option<bool>,
    pub description: Option<String>,
    pub workspace_id: Option<WorkspaceId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarUpdateRequest {
    pub calendars: Vec<CalendarUpdate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarUpdate {
    pub id: CalendarId,
    pub selected: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateIntegration {
    pub enabled: Option<bool>,
}
