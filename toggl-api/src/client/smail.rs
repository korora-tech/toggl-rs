use super::TogglClient;
use crate::models::api::smail::{ContactRequest, DemoRequest, MeetRequest};
use reqwest::Method;
use toggl_core::Result;

pub struct SmailClient {
    client: TogglClient,
}

impl SmailClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Send contact request
    pub fn contact(&self, request: &ContactRequest) -> Result<()> {
        self.client
            .request_with_body_empty(Method::POST, "smail/contact", request)
    }

    /// Request a demo
    pub fn demo(&self, request: &DemoRequest) -> Result<()> {
        self.client
            .request_with_body_empty(Method::POST, "smail/demo", request)
    }

    /// Schedule a meeting
    pub fn meet(&self, request: &MeetRequest) -> Result<()> {
        self.client
            .request_with_body_empty(Method::POST, "smail/meet", request)
    }
}
