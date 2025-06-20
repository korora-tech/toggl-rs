use super::TogglClient;
use crate::models::reports::*;
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;
use toggl_core::WorkspaceId;

pub struct ReportsClient {
    client: TogglClient,
}

impl ReportsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get summary report
    pub fn summary(
        &self,
        workspace_id: WorkspaceId,
        params: BTreeMap<String, String>,
    ) -> Result<summary::SummaryReportResponse> {
        self.client.request_with_params(
            Method::POST,
            &format!("reports/api/v3/workspace/{}/summary", workspace_id),
            &params,
        )
    }

    /// Get detailed report
    pub fn detailed(
        &self,
        workspace_id: WorkspaceId,
        params: BTreeMap<String, String>,
    ) -> Result<detailed::DetailedReportResponse> {
        self.client.request_with_params(
            Method::POST,
            &format!("reports/api/v3/workspace/{}/detailed", workspace_id),
            &params,
        )
    }

    /// Get weekly report
    pub fn weekly(
        &self,
        workspace_id: WorkspaceId,
        params: BTreeMap<String, String>,
    ) -> Result<weekly::WeeklyReportResponse> {
        self.client.request_with_params(
            Method::POST,
            &format!("reports/api/v3/workspace/{}/weekly", workspace_id),
            &params,
        )
    }

    /// Get project trends report
    pub fn project_trends(
        &self,
        workspace_id: WorkspaceId,
        params: BTreeMap<String, String>,
    ) -> Result<trends::ProjectTrendsResponse> {
        self.client.request_with_params(
            Method::POST,
            &format!("reports/api/v3/workspace/{}/trends/projects", workspace_id),
            &params,
        )
    }

    /// Get client trends report
    pub fn client_trends(
        &self,
        workspace_id: WorkspaceId,
        params: BTreeMap<String, String>,
    ) -> Result<trends::ClientTrendsResponse> {
        self.client.request_with_params(
            Method::POST,
            &format!("reports/api/v3/workspace/{}/trends/clients", workspace_id),
            &params,
        )
    }

    /// Get user trends report
    pub fn user_trends(
        &self,
        workspace_id: WorkspaceId,
        params: BTreeMap<String, String>,
    ) -> Result<trends::UserTrendsResponse> {
        self.client.request_with_params(
            Method::POST,
            &format!("reports/api/v3/workspace/{}/trends/users", workspace_id),
            &params,
        )
    }

    /// Get project profitability report
    pub fn project_profitability(
        &self,
        workspace_id: WorkspaceId,
        params: BTreeMap<String, String>,
    ) -> Result<profitability::ProjectProfitabilityResponse> {
        self.client.request_with_params(
            Method::POST,
            &format!(
                "reports/api/v3/workspace/{}/profitability/projects",
                workspace_id
            ),
            &params,
        )
    }

    /// Get employee profitability report
    pub fn employee_profitability(
        &self,
        workspace_id: WorkspaceId,
        params: BTreeMap<String, String>,
    ) -> Result<profitability::EmployeeProfitabilityResponse> {
        self.client.request_with_params(
            Method::POST,
            &format!(
                "reports/api/v3/workspace/{}/profitability/employees",
                workspace_id
            ),
            &params,
        )
    }

    /// Get comparative report
    pub fn comparative(
        &self,
        workspace_id: WorkspaceId,
        params: BTreeMap<String, String>,
    ) -> Result<comparative::ComparativeReportResponse> {
        self.client.request_with_params(
            Method::POST,
            &format!("reports/api/v3/workspace/{}/comparative", workspace_id),
            &params,
        )
    }

    /// Export report
    pub fn export(
        &self,
        workspace_id: WorkspaceId,
        export_type: &str,
        params: BTreeMap<String, String>,
    ) -> Result<export::ExportResponse> {
        let mut params = params;
        params.insert("export_type".to_string(), export_type.to_string());

        self.client.request_with_params(
            Method::POST,
            &format!("reports/api/v3/workspace/{}/export", workspace_id),
            &params,
        )
    }
}
