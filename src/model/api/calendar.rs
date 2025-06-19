use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Integration {
    pub id: u64,
    pub provider: String,
    pub provider_user_id: String,
    pub linked_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Calendar {
    pub id: String,
    pub integration_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub selected: bool,
    pub color: Option<String>,
    pub ical_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarEvent {
    pub id: String,
    pub calendar_id: String,
    pub integration_id: u64,
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
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
    pub billable: Option<bool>,
    pub description: Option<String>,
    pub workspace_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarUpdateRequest {
    pub calendars: Vec<CalendarUpdate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarUpdate {
    pub id: String,
    pub selected: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateIntegration {
    pub enabled: Option<bool>,
}
