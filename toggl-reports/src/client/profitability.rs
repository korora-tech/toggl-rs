//! Profitability report endpoints

use reqwest::Method;
use toggl_core::Result;

use super::ReportsClient;
use crate::models::profitability::*;

pub struct ProfitabilityClient {
    client: ReportsClient,
}

impl ProfitabilityClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Get project profitability report
    pub fn projects(
        &self,
        workspace_id: i64,
        request: &ProjectProfitabilityRequest,
    ) -> Result<ProjectProfitabilityReport> {
        let path = format!("/workspace/{}/profitability/projects", workspace_id);
        self.client.request_with_body(Method::POST, &path, request)
    }
}
