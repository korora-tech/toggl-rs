use super::TogglClient;
use crate::models::api::country::{Country, Subdivision};
use reqwest::Method;
use toggl_core::Result;

pub struct CountriesClient {
    client: TogglClient,
}

impl CountriesClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get list of countries
    pub fn list(&self) -> Result<Vec<Country>> {
        self.client.request(Method::GET, "countries")
    }

    /// Get country subdivisions
    pub fn get_subdivisions(&self, country_id: u64) -> Result<Vec<Subdivision>> {
        self.client.request(
            Method::GET,
            &format!("countries/{}/subdivisions", country_id),
        )
    }
}
