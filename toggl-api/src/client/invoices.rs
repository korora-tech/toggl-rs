use super::TogglClient;
use crate::models::api::invoices::*;
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;

pub struct InvoicesClient {
    client: TogglClient,
}

impl InvoicesClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get organization invoice PDF
    pub fn get_organization_invoice_pdf(
        &self,
        organization_id: u64,
        invoice_uid: &str,
    ) -> Result<Vec<u8>> {
        self.client.request_binary(
            Method::GET,
            &format!(
                "organizations/{}/invoices/{}.pdf",
                organization_id, invoice_uid
            ),
        )
    }

    /// Get organization paid invoices
    pub fn get_organization_invoices(
        &self,
        organization_id: u64,
        next_cursor: Option<&str>,
    ) -> Result<UnifiedSubscriptionInvoiceList> {
        let mut params = BTreeMap::new();
        if let Some(cursor) = next_cursor {
            params.insert("next".to_string(), cursor.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("organizations/{}/subscription/invoices", organization_id),
            &params,
        )
    }

    /// Get organization invoice summary
    pub fn get_organization_invoice_summary(&self, organization_id: u64) -> Result<String> {
        self.client.request(
            Method::GET,
            &format!(
                "organizations/{}/subscription/invoice_summary",
                organization_id
            ),
        )
    }

    /// Get workspace invoices
    pub fn get_workspace_invoices(
        &self,
        workspace_id: u64,
        page: Option<u32>,
        per_page: Option<u32>,
        sort_field: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<UserInvoicesResponse> {
        let mut params = BTreeMap::new();
        if let Some(page) = page {
            params.insert("page".to_string(), page.to_string());
        }
        if let Some(per_page) = per_page {
            params.insert("per_page".to_string(), per_page.to_string());
        }
        if let Some(sort_field) = sort_field {
            params.insert("sort_field".to_string(), sort_field.to_string());
        }
        if let Some(sort_order) = sort_order {
            params.insert("sort_order".to_string(), sort_order.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/invoices", workspace_id),
            &params,
        )
    }

    /// Create workspace user invoice
    pub fn create_workspace_invoice(
        &self,
        workspace_id: u64,
        invoice: &CreateUserInvoice,
    ) -> Result<UserInvoice> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/invoices", workspace_id),
            invoice,
        )
    }

    /// Get workspace invoice PDF
    pub fn get_workspace_invoice_pdf(&self, workspace_id: u64, invoice_id: u64) -> Result<Vec<u8>> {
        self.client.request_binary(
            Method::GET,
            &format!("workspaces/{}/invoices/{}.pdf", workspace_id, invoice_id),
        )
    }

    /// Delete workspace user invoice
    pub fn delete_workspace_invoice(&self, workspace_id: u64, user_invoice_id: u64) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/invoices/{}", workspace_id, user_invoice_id),
        )
    }
}
