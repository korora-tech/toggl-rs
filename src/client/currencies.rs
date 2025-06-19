use super::TogglClient;
use crate::{error::Result, model::api::*};
use reqwest::Method;

pub struct CurrenciesClient {
    client: TogglClient,
}

impl CurrenciesClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get list of currencies
    pub fn list(&self) -> Result<Vec<Currency>> {
        self.client.request(Method::GET, "currencies")
    }
}
