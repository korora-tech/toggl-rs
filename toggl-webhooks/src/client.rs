//! Toggl Webhooks API client implementation

use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::BTreeMap;
use toggl_core::{BaseClient, Result, WorkspaceId};

use crate::models::*;

/// Base URL for the Toggl Webhooks API
pub const WEBHOOKS_API_BASE_URL: &str = "https://api.track.toggl.com/webhooks";

/// Toggl Webhooks API client
#[derive(Clone)]
pub struct WebhooksClient {
    base_url: String,
    base_client: BaseClient,
}

impl WebhooksClient {
    /// Create a new Webhooks API client
    pub fn new(api_token: String) -> Result<Self> {
        Self::new_with_base_url(api_token, WEBHOOKS_API_BASE_URL)
    }

    /// Create a new Webhooks API client with a custom base URL
    pub fn new_with_base_url(api_token: String, base_url: &str) -> Result<Self> {
        let base_client = BaseClient::new(api_token)?;
        Ok(Self {
            base_url: base_url.to_string(),
            base_client,
        })
    }

    /// Build a URL for the API endpoint
    fn url(&self, path: &str) -> String {
        format!("{}/api/v9{}", self.base_url, path)
    }

    /// Send a request to the API
    fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        params: Option<&BTreeMap<String, String>>,
        body: Option<impl Serialize>,
    ) -> Result<T> {
        let url = self.url(path);
        match (params, body) {
            (Some(params), None) => self.base_client.request_with_params(method, &url, params),
            (None, Some(body)) => self.base_client.request_with_body(method, &url, &body),
            (Some(_), Some(_)) => {
                // For requests with both params and body, we need to handle differently
                // This is not directly supported by BaseClient, so we'll need to adjust
                unimplemented!("Requests with both params and body are not yet supported")
            }
            (None, None) => self.base_client.request(method, &url),
        }
    }

    /// Send a request that returns no content
    fn request_empty(
        &self,
        method: Method,
        path: &str,
        params: Option<&BTreeMap<String, String>>,
        body: Option<impl Serialize>,
    ) -> Result<()> {
        let url = self.url(path);
        match (params, body) {
            (None, Some(body)) => self
                .base_client
                .request_with_body_empty(method, &url, &body),
            _ => self.base_client.request_empty(method, &url),
        }
    }

    // Subscription Management

    /// Get workspace subscriptions
    ///
    /// # Arguments
    /// * `workspace_id` - Numeric ID of the workspace
    pub fn get_subscriptions(&self, workspace_id: WorkspaceId) -> Result<Vec<Subscription>> {
        let mut params = BTreeMap::new();
        params.insert("workspace_id".to_string(), workspace_id.to_string());

        self.request(Method::GET, "/subscriptions", Some(&params), None::<()>)
    }

    /// Create a subscription
    ///
    /// # Arguments
    /// * `subscription` - Subscription details
    pub fn create_subscription(&self, subscription: &CreateSubscription) -> Result<Subscription> {
        self.request(Method::POST, "/subscriptions", None, Some(subscription))
    }

    /// Get a subscription by ID
    ///
    /// # Arguments
    /// * `subscription_id` - Subscription ID
    pub fn get_subscription(&self, subscription_id: &str) -> Result<Subscription> {
        let path = format!("/subscriptions/{}", subscription_id);
        self.request(Method::GET, &path, None, None::<()>)
    }

    /// Update a subscription
    ///
    /// # Arguments
    /// * `subscription_id` - Subscription ID
    /// * `update` - Update details
    pub fn update_subscription(
        &self,
        subscription_id: &str,
        update: &UpdateSubscription,
    ) -> Result<Subscription> {
        let path = format!("/subscriptions/{}", subscription_id);
        self.request(Method::PUT, &path, None, Some(update))
    }

    /// Delete a subscription
    ///
    /// # Arguments
    /// * `subscription_id` - Subscription ID
    pub fn delete_subscription(&self, subscription_id: &str) -> Result<()> {
        let path = format!("/subscriptions/{}", subscription_id);
        self.request_empty(Method::DELETE, &path, None, None::<()>)
    }

    // Event Filters

    /// Get available event filters
    pub fn get_event_filters(&self) -> Result<EventFilters> {
        self.request(Method::GET, "/event_filters", None, None::<()>)
    }

    // Ping

    /// Send a test event to a subscription
    ///
    /// # Arguments
    /// * `subscription_id` - Subscription ID
    pub fn ping_subscription(&self, subscription_id: &str) -> Result<()> {
        let path = format!("/ping/{}", subscription_id);
        self.request_empty(Method::POST, &path, None, None::<()>)
    }

    // Health

    /// Get application health status
    pub fn get_health(&self) -> Result<AppHealth> {
        self.request(Method::GET, "/health", None, None::<()>)
    }

    // Subscription Validation

    /// Validate subscription URLs
    ///
    /// This endpoint is typically called by your webhook handler to validate
    /// incoming subscription validation requests from Toggl.
    ///
    /// # Arguments
    /// * `validation` - Validation request details
    pub fn validate_subscription(
        &self,
        validation: &ValidationRequest,
    ) -> Result<ValidationResponse> {
        // Note: This endpoint doesn't follow the standard /api/v9 prefix
        let url = format!("{}/subscription_validation_events", self.base_url);
        self.base_client
            .request_with_body(Method::POST, &url, validation)
    }
}
