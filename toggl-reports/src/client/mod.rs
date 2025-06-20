//! Toggl Reports API client implementation

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::BTreeMap;

use toggl_core::{BaseClient, Result};

pub mod comparative;
pub mod data_trends;
pub mod filters;
pub mod insights;
pub mod profitability;
pub mod projects;
pub mod search;
pub mod shared;
pub mod summary;
pub mod weekly;

const REPORTS_BASE_URL: &str = "https://api.track.toggl.com/reports/api/v3";
const INSIGHTS_BASE_URL: &str = "https://api.track.toggl.com/insights/api/v1";

#[derive(Clone)]
pub struct ReportsClient {
    reports_base_url: String,
    insights_base_url: String,
    base_client: BaseClient,
}

impl ReportsClient {
    pub fn new(api_token: String) -> Result<Self> {
        Self::new_with_base_urls(api_token, REPORTS_BASE_URL, INSIGHTS_BASE_URL)
    }

    pub fn new_with_base_urls(
        api_token: String,
        reports_base_url: &str,
        insights_base_url: &str,
    ) -> Result<Self> {
        let base_client = BaseClient::new(api_token)?;
        Ok(Self {
            reports_base_url: reports_base_url.to_string(),
            insights_base_url: insights_base_url.to_string(),
            base_client,
        })
    }

    pub(crate) fn request<T: DeserializeOwned>(&self, method: Method, path: &str) -> Result<T> {
        self.request_with_params(method, path, &BTreeMap::new())
    }

    pub(crate) fn request_with_params<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        params: &BTreeMap<String, String>,
    ) -> Result<T> {
        let url = self.build_url(path);
        self.base_client.request_with_params(method, &url, params)
    }

    pub(crate) fn request_with_body<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path);
        self.base_client.request_with_body(method, &url, body)
    }

    pub(crate) fn request_bytes_with_body<B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<Vec<u8>> {
        let url = self.build_url(path);
        self.base_client.request_bytes_with_body(method, &url, body)
    }

    pub(crate) fn request_bytes(&self, method: Method, path: &str) -> Result<Vec<u8>> {
        let url = self.build_url(path);
        self.base_client.request_binary(method, &url)
    }

    fn build_url(&self, path: &str) -> String {
        let (base_url, clean_path) = if path.starts_with("/insights") {
            (
                &self.insights_base_url,
                path.trim_start_matches("/insights"),
            )
        } else {
            (&self.reports_base_url, path)
        };

        if clean_path.starts_with('/') {
            format!("{}{}", base_url, clean_path)
        } else {
            format!("{}/{}", base_url, clean_path)
        }
    }
}

// Client accessors for different API domains
impl ReportsClient {
    /// Access summary report endpoints
    pub fn summary(&self) -> summary::SummaryClient {
        summary::SummaryClient::new(self.clone())
    }

    /// Access detailed report endpoints
    pub fn detailed(&self) -> search::DetailedClient {
        search::DetailedClient::new(self.clone())
    }

    /// Access weekly report endpoints
    pub fn weekly(&self) -> weekly::WeeklyClient {
        weekly::WeeklyClient::new(self.clone())
    }

    /// Access comparative report endpoints
    pub fn comparative(&self) -> comparative::ComparativeClient {
        comparative::ComparativeClient::new(self.clone())
    }

    /// Access data trends endpoints
    pub fn data_trends(&self) -> data_trends::DataTrendsClient {
        data_trends::DataTrendsClient::new(self.clone())
    }

    /// Access profitability endpoints
    pub fn profitability(&self) -> profitability::ProfitabilityClient {
        profitability::ProfitabilityClient::new(self.clone())
    }

    /// Access filter endpoints
    pub fn filters(&self) -> filters::FiltersClient {
        filters::FiltersClient::new(self.clone())
    }

    /// Access search endpoints
    pub fn search(&self) -> search::SearchClient {
        search::SearchClient::new(self.clone())
    }

    /// Access shared report endpoints
    pub fn shared(&self) -> shared::SharedClient {
        shared::SharedClient::new(self.clone())
    }

    /// Access project report endpoints
    pub fn projects(&self) -> projects::ProjectsClient {
        projects::ProjectsClient::new(self.clone())
    }

    /// Access insights API endpoints
    pub fn insights(&self) -> insights::InsightsClient {
        insights::InsightsClient::new(self.clone())
    }
}
