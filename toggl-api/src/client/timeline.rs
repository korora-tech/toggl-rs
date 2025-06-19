use super::TogglClient;
use crate::models::api::timeline::{CreateTimelineEvent, TimelineEvent};
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;

pub struct TimelineClient {
    client: TogglClient,
}

impl TimelineClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get timeline events
    pub fn list(&self, start_date: &str, end_date: &str) -> Result<Vec<TimelineEvent>> {
        let mut params = BTreeMap::new();
        params.insert("start_date".to_string(), start_date.to_string());
        params.insert("end_date".to_string(), end_date.to_string());

        self.client
            .request_with_params(Method::GET, "timeline", &params)
    }

    /// Create timeline event
    pub fn create(&self, event: &CreateTimelineEvent) -> Result<TimelineEvent> {
        self.client
            .request_with_body(Method::POST, "timeline", event)
    }

    /// Delete timeline event
    pub fn delete(&self, event_id: u64) -> Result<()> {
        self.client
            .request_empty(Method::DELETE, &format!("timeline/{}", event_id))
    }
}
