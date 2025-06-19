use super::TogglClient;
use crate::models::api::subscription::SubscriptionPlan;
use reqwest::Method;
use toggl_core::Result;

pub struct SubscriptionsClient {
    client: TogglClient,
}

impl SubscriptionsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get available subscription plans
    pub fn plans(&self) -> Result<Vec<SubscriptionPlan>> {
        self.client.request(Method::GET, "subscriptions/plans")
    }
}
