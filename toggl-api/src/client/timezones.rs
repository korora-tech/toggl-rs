use super::TogglClient;
use crate::models::api::timezones::{Timezone, TimezoneOffset};
use reqwest::Method;
use toggl_core::Result;

pub struct TimezonesClient {
    client: TogglClient,
}

impl TimezonesClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get list of timezones
    pub fn list(&self) -> Result<Vec<Timezone>> {
        self.client.request(Method::GET, "timezones")
    }

    /// Get timezone offsets
    pub fn get_offsets(&self) -> Result<Vec<TimezoneOffset>> {
        self.client.request(Method::GET, "timezones/offsets")
    }
}
