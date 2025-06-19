use super::TogglClient;
use crate::models::api::calendar::*;
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;

pub struct CalendarClient {
    client: TogglClient,
}

impl CalendarClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get calendar integrations
    pub fn get_integrations(&self) -> Result<Vec<Integration>> {
        self.client.request(Method::GET, "integrations/calendar")
    }

    /// Get calendars
    pub fn get_calendars(&self) -> Result<CalendarsResponse> {
        self.client
            .request(Method::GET, "integrations/calendar/calendars")
    }

    /// Get selected calendars
    pub fn get_selected_calendars(&self) -> Result<Vec<Calendar>> {
        self.client
            .request(Method::GET, "integrations/calendar/calendars/selected")
    }

    /// Get calendar events
    pub fn get_events(&self, start_date: &str, end_date: &str) -> Result<EventsResponse> {
        let mut params = BTreeMap::new();
        params.insert("start_date".to_string(), start_date.to_string());
        params.insert("end_date".to_string(), end_date.to_string());

        self.client
            .request_with_params(Method::GET, "integrations/calendar/events", &params)
    }

    /// Update calendar event
    pub fn update_event(&self, event: &CalendarEvent) -> Result<CalendarEvent> {
        self.client
            .request_with_body(Method::POST, "integrations/calendar/events/update", event)
    }

    /// Setup calendar integration
    pub fn setup(&self, provider: &str) -> Result<Integration> {
        let mut body = BTreeMap::new();
        body.insert("provider", provider);

        self.client
            .request_with_body(Method::POST, "integrations/calendar/setup", &body)
    }

    /// Delete calendar integration
    pub fn delete_integration(&self, integration_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("integrations/calendar/{}", integration_id),
        )
    }

    /// Update calendar integration
    pub fn update_integration(
        &self,
        integration_id: u64,
        update: &UpdateIntegration,
    ) -> Result<Integration> {
        self.client.request_with_body(
            Method::PUT,
            &format!("integrations/calendar/{}", integration_id),
            update,
        )
    }

    /// Get calendars for specific integration
    pub fn get_integration_calendars(&self, integration_id: u64) -> Result<CalendarsResponse> {
        self.client.request(
            Method::GET,
            &format!("integrations/calendar/{}/calendars", integration_id),
        )
    }

    /// Update calendars for specific integration
    pub fn update_integration_calendars(
        &self,
        integration_id: u64,
        update: &CalendarUpdateRequest,
    ) -> Result<()> {
        self.client.request_with_body_empty(
            Method::POST,
            &format!("integrations/calendar/{}/calendars/update", integration_id),
            update,
        )
    }

    /// Update specific calendar
    pub fn update_calendar(
        &self,
        integration_id: u64,
        calendar_id: &str,
        update: &CalendarUpdate,
    ) -> Result<Calendar> {
        self.client.request_with_body(
            Method::PATCH,
            &format!(
                "integrations/calendar/{}/calendars/{}",
                integration_id, calendar_id
            ),
            update,
        )
    }

    /// Get events for specific calendar
    pub fn get_calendar_events(
        &self,
        integration_id: u64,
        calendar_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<EventsResponse> {
        let mut params = BTreeMap::new();
        params.insert("start_date".to_string(), start_date.to_string());
        params.insert("end_date".to_string(), end_date.to_string());

        self.client.request_with_params(
            Method::GET,
            &format!(
                "integrations/calendar/{}/calendars/{}/events",
                integration_id, calendar_id
            ),
            &params,
        )
    }

    /// Get event details suggestion
    pub fn get_event_details_suggestion(&self, event_id: &str) -> Result<EventDetailsSuggestion> {
        self.client.request(
            Method::GET,
            &format!(
                "integrations/calendar/events/{}/details-suggestion",
                event_id
            ),
        )
    }

    /// Get event details suggestions (batch)
    pub fn get_events_details_suggestions(
        &self,
        event_ids: &[String],
    ) -> Result<Vec<EventDetailsSuggestion>> {
        let body = serde_json::json!({
            "event_ids": event_ids
        });

        self.client.request_with_body(
            Method::POST,
            "integrations/calendar/events/details-suggestion",
            &body,
        )
    }

    /// OAuth callback for calendar provider
    pub fn calendar_callback(
        &self,
        provider: &str,
        code: &str,
        state: Option<&str>,
    ) -> Result<Integration> {
        let mut params = BTreeMap::new();
        params.insert("code".to_string(), code.to_string());
        if let Some(state) = state {
            params.insert("state".to_string(), state.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("integrations/calendar/callback/{}", provider),
            &params,
        )
    }
}
