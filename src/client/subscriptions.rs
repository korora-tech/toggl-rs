use super::TogglClient;
use crate::error::Result;
use crate::model::api::subscription::SubscriptionPlan;
use reqwest::Method;

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
