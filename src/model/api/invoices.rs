use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInvoice {
    pub billing_address: Option<String>,
    pub created_at: Option<String>,
    pub currency: Option<String>,
    pub date: Option<String>,
    pub deleted_at: Option<String>,
    pub document_id: Option<String>,
    pub due_date: Option<String>,
    pub integration_ext_id: Option<String>,
    pub integration_ext_type: Option<String>,
    pub integration_provider: Option<IntegrationProvider>,
    pub items: Option<Vec<UserInvoiceItem>>,
    pub message: Option<String>,
    pub payment_terms: Option<String>,
    pub purchase_number: Option<String>,
    pub taxes: Option<Vec<UserInvoiceTax>>,
    pub updated_at: Option<String>,
    pub user_id: Option<i64>,
    pub user_invoice_id: Option<i64>,
    pub workspace_address: Option<String>,
    pub workspace_id: Option<i64>,
    pub workspace_logo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInvoiceItem {
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub item_id: Option<i64>,
    pub quantity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInvoiceTax {
    pub amount: Option<f64>,
    pub name: Option<String>,
    pub tax_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSubscriptionInvoice {
    pub creation_date: Option<String>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub hosted_url: Option<String>,
    pub id: Option<String>,
    pub invoice_number: Option<String>,
    pub pdf_url: Option<String>,
    pub status: Option<String>,
    pub total_amount: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSubscriptionInvoiceList {
    pub items: Option<Vec<UnifiedSubscriptionInvoice>>,
    pub next: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntegrationProvider {
    Asana,
    Jira,
    Salesforce,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserInvoice {
    pub billing_address: Option<String>,
    pub currency: Option<String>,
    pub date: Option<String>,
    pub due_date: Option<String>,
    pub integration_ext_id: Option<String>,
    pub integration_ext_type: Option<String>,
    pub integration_provider: Option<IntegrationProvider>,
    pub items: Option<Vec<UserInvoiceItem>>,
    pub message: Option<String>,
    pub payment_terms: Option<String>,
    pub purchase_number: Option<String>,
    pub taxes: Option<Vec<UserInvoiceTax>>,
    pub workspace_address: Option<String>,
    pub workspace_logo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInvoicesResponse {
    pub data: Option<Vec<UserInvoice>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub total_count: Option<i32>,
}
