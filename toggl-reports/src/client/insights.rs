//! Insights API endpoints

use reqwest::Method;
use toggl_core::{Result, WorkspaceId};

use super::ReportsClient;
use crate::models::profitability::*;

pub struct InsightsClient {
    client: ReportsClient,
}

impl InsightsClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Get project trends from insights API
    pub fn project_trends(
        &self,
        workspace_id: WorkspaceId,
        request: &ProjectTrendsRequest,
    ) -> Result<Vec<ProjectTrends>> {
        let path = format!(
            "/insights/api/v1/workspace/{}/data_trends/projects",
            workspace_id.value()
        );
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Export employee profitability as CSV
    pub fn employee_profitability_csv(
        &self,
        workspace_id: WorkspaceId,
        request: &EmployeeProfitabilityRequest,
    ) -> Result<Vec<u8>> {
        let path = format!(
            "/insights/api/v1/workspace/{}/profitability/employees.csv",
            workspace_id.value()
        );
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }

    /// Export employee profitability as XLSX
    pub fn employee_profitability_xlsx(
        &self,
        workspace_id: WorkspaceId,
        request: &EmployeeProfitabilityRequest,
    ) -> Result<Vec<u8>> {
        let path = format!(
            "/insights/api/v1/workspace/{}/profitability/employees.xlsx",
            workspace_id.value()
        );
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }

    /// Export project profitability as CSV
    pub fn project_profitability_csv(
        &self,
        workspace_id: WorkspaceId,
        request: &ProjectTrendsRequest,
    ) -> Result<Vec<u8>> {
        let path = format!(
            "/insights/api/v1/workspace/{}/profitability/projects.csv",
            workspace_id.value()
        );
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }

    /// Export project profitability as XLSX
    pub fn project_profitability_xlsx(
        &self,
        workspace_id: WorkspaceId,
        request: &ProjectTrendsRequest,
    ) -> Result<Vec<u8>> {
        let path = format!(
            "/insights/api/v1/workspace/{}/profitability/projects.xlsx",
            workspace_id.value()
        );
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }

    /// Export project trends as CSV
    pub fn project_trends_csv(
        &self,
        workspace_id: WorkspaceId,
        request: &ProjectTrendsRequest,
    ) -> Result<Vec<u8>> {
        let path = format!(
            "/insights/api/v1/workspace/{}/trends/projects.csv",
            workspace_id.value()
        );
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }

    /// Export project trends as XLSX
    pub fn project_trends_xlsx(
        &self,
        workspace_id: WorkspaceId,
        request: &ProjectTrendsRequest,
    ) -> Result<Vec<u8>> {
        let path = format!(
            "/insights/api/v1/workspace/{}/trends/projects.xlsx",
            workspace_id.value()
        );
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }
}
