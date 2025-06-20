//! Comparative report endpoints

use reqwest::Method;
use toggl_core::Result;

use super::ReportsClient;
use crate::models::comparative::*;

pub struct ComparativeClient {
    client: ReportsClient,
}

impl ComparativeClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Get comparative report for a workspace
    pub fn get(&self, workspace_id: i64, request: &ComparativePost) -> Result<ComparativeReport> {
        let path = format!("/workspace/{}/comparative", workspace_id);
        self.client.request_with_body(Method::POST, &path, request)
    }
}
