use super::TogglClient;
use crate::error::Result;
use crate::model::api::AuditLog;
use chrono::{DateTime, Utc};
use reqwest::Method;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct AuditLogFilters {
    pub workspace_id: Option<u64>,
    pub entity_type: Option<String>,
    pub entity_id: Option<u64>,
    pub action: Option<String>,
    pub user_id: Option<u64>,
    pub page_size: Option<u64>,
    pub offset: Option<u64>,
}

pub struct AuditClient {
    client: TogglClient,
}

impl AuditClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get audit logs for an organization
    pub fn get_logs(
        &self,
        organization_id: u64,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<AuditLog>> {
        let path = format!(
            "audit_logs/{}/{}/{}",
            organization_id,
            from.to_rfc3339(),
            to.to_rfc3339()
        );
        self.client.request(Method::GET, &path)
    }

    /// Get audit logs with filters
    pub fn get_logs_with_filters(
        &self,
        organization_id: u64,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        filters: &AuditLogFilters,
    ) -> Result<Vec<AuditLog>> {
        let path = format!(
            "audit_logs/{}/{}/{}",
            organization_id,
            from.to_rfc3339(),
            to.to_rfc3339()
        );

        let mut params = BTreeMap::new();
        if let Some(workspace_id) = filters.workspace_id {
            params.insert("workspace_id".to_string(), workspace_id.to_string());
        }
        if let Some(entity_type) = &filters.entity_type {
            params.insert("entity_type".to_string(), entity_type.clone());
        }
        if let Some(entity_id) = filters.entity_id {
            params.insert("entity_id".to_string(), entity_id.to_string());
        }
        if let Some(action) = &filters.action {
            params.insert("action".to_string(), action.clone());
        }
        if let Some(user_id) = filters.user_id {
            params.insert("user_id".to_string(), user_id.to_string());
        }
        if let Some(page_size) = filters.page_size {
            params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(offset) = filters.offset {
            params.insert("offset".to_string(), offset.to_string());
        }

        self.client.request_with_params(Method::GET, &path, &params)
    }

    /// Export all audit logs without pagination
    pub fn export_logs(
        &self,
        organization_id: u64,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<AuditLog>> {
        let path = format!(
            "audit_logs/{}/{}/{}",
            organization_id,
            from.to_rfc3339(),
            to.to_rfc3339()
        );

        let mut params = BTreeMap::new();
        params.insert("export".to_string(), "true".to_string());

        self.client.request_with_params(Method::GET, &path, &params)
    }

    /// Get audit logs with pagination
    pub fn get_logs_paginated(
        &self,
        organization_id: u64,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        page_size: u64,
    ) -> Result<Vec<AuditLog>> {
        let path = format!(
            "audit_logs/{}/{}/{}",
            organization_id,
            from.to_rfc3339(),
            to.to_rfc3339()
        );

        let mut params = BTreeMap::new();
        params.insert("page_size".to_string(), page_size.to_string());

        self.client.request_with_params(Method::GET, &path, &params)
    }
}
