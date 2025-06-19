use super::TogglClient;
use crate::models::api::feedback::{FeedbackRequest, WebFeedbackRequest};
use reqwest::Method;
use toggl_core::Result;

pub struct FeedbackClient {
    client: TogglClient,
}

impl FeedbackClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Send feedback
    pub fn send(&self, feedback: &FeedbackRequest) -> Result<()> {
        self.client
            .request_with_body_empty(Method::POST, "feedback", feedback)
    }

    /// Send web feedback
    pub fn send_web(&self, feedback: &WebFeedbackRequest) -> Result<()> {
        self.client
            .request_with_body_empty(Method::POST, "feedback/web", feedback)
    }

    /// Send mobile feedback
    pub fn send_mobile(&self, feedback: &FeedbackRequest) -> Result<()> {
        self.client
            .request_with_body_empty(Method::POST, "mobile/feedback", feedback)
    }
}
