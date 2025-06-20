//! Search and detailed report endpoints

use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::{Result, WorkspaceId};

use super::ReportsClient;
use crate::models::detailed::*;
use crate::models::search::*;

pub struct DetailedClient {
    client: ReportsClient,
}

impl DetailedClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Get detailed time entries report
    pub fn get(&self, workspace_id: WorkspaceId, request: &DetailedPost) -> Result<DetailedReport> {
        let path = format!("/workspace/{}/search/time_entries", workspace_id.value());
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Get time entries totals
    pub fn totals(
        &self,
        workspace_id: WorkspaceId,
        request: &DetailedPost,
    ) -> Result<DetailedTotals> {
        let path = format!(
            "/workspace/{}/search/time_entries/totals",
            workspace_id.value()
        );
        self.client.request_with_body(Method::POST, &path, request)
    }

    /// Export detailed report as CSV
    pub fn export_csv(
        &self,
        workspace_id: WorkspaceId,
        request: &SearchExportPost,
    ) -> Result<Vec<u8>> {
        let path = format!(
            "/workspace/{}/search/time_entries.csv",
            workspace_id.value()
        );
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }

    /// Export detailed report as XLSX
    pub fn export_xlsx(
        &self,
        workspace_id: WorkspaceId,
        request: &SearchExportPost,
    ) -> Result<Vec<u8>> {
        let path = format!(
            "/workspace/{}/search/time_entries.xlsx",
            workspace_id.value()
        );
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }

    /// Export detailed report as PDF
    pub fn export_pdf(
        &self,
        workspace_id: WorkspaceId,
        request: &ExportPDFPost,
    ) -> Result<Vec<u8>> {
        let path = format!(
            "/workspace/{}/search/time_entries.pdf",
            workspace_id.value()
        );
        self.client
            .request_bytes_with_body(Method::POST, &path, request)
    }
}

pub struct SearchClient {
    client: ReportsClient,
}

impl SearchClient {
    pub fn new(client: ReportsClient) -> Self {
        Self { client }
    }

    /// Search clients
    pub fn clients(&self, workspace_id: WorkspaceId, query: &str) -> Result<SearchClientsResponse> {
        let path = format!("/workspace/{}/search/clients", workspace_id.value());
        let mut params = BTreeMap::new();
        params.insert("q".to_string(), query.to_string());
        self.client.request_with_params(Method::GET, &path, &params)
    }

    /// Search projects
    pub fn projects(
        &self,
        workspace_id: WorkspaceId,
        query: &str,
    ) -> Result<SearchProjectsResponse> {
        let path = format!("/workspace/{}/search/projects", workspace_id.value());
        let mut params = BTreeMap::new();
        params.insert("q".to_string(), query.to_string());
        self.client.request_with_params(Method::GET, &path, &params)
    }

    /// Search users
    pub fn users(&self, workspace_id: WorkspaceId, query: &str) -> Result<SearchUsersResponse> {
        let path = format!("/workspace/{}/search/users", workspace_id.value());
        let mut params = BTreeMap::new();
        params.insert("q".to_string(), query.to_string());
        self.client.request_with_params(Method::GET, &path, &params)
    }
}
