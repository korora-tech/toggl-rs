use super::ids::{
    CompanyId, ContactDetailId, CountryId, CountrySubdivisionId, CreatorId, CurrencyId, CustomerId,
    PricingPlanId, SubscriptionId, SubscriptionPeriodId, UserId,
};
use serde::{Deserialize, Serialize};

/// Workspace subscription response from API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSubscriptionResponse {
    pub active_users: i32,
    pub auto_renew: bool,
    pub billing_period_in_months: i32,
    pub campaign_available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_details: Option<SubscriptionCardDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<CompanyId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<SubscriptionContactDetail>,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_ends_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_starts_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>, // deprecated
    pub enterprise: bool,
    pub is_subscription_beta: bool,
    pub is_unified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_invoice: Option<InvoiceInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment: Option<PaymentInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pricing_plan_id: Option<PricingPlanId>,
    pub new_signup_trial: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_date: Option<String>,
    pub payment_failed: bool,
    pub payment_method: String,
    pub plan_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan_id: Option<PricingPlanId>, // legacy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_date: Option<String>,
    pub seat_cost_in_cents: i64,
    pub seats: i32,
    pub site: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>, // deprecated
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<SubscriptionPeriod>,
    pub trial_available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_start_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCardDetails {
    pub added_at: String,
    pub card_number: String,
    pub card_type: String,
    pub creator_id: CreatorId,
    pub creator_name: String,
    pub expiry_date: String,
    pub holder_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionContactDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    pub contact_detail_id: ContactDetailId,
    pub contact_email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_person: Option<String>,
    pub country_id: CountryId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_subdivision_id: Option<CountrySubdivisionId>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    pub is_eu_resident: bool,
    pub updated_at: String,
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number_valid: Option<bool>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number_validated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceInfo {
    pub amount: i64,
    pub created_at: String,
    pub currency_id: CurrencyId,
    pub due: String,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    pub tax_percentage: f64,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInfo {
    pub created_at: String,
    pub description: String,
    pub id: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPeriod {
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_on: Option<String>,
    pub started_on: String,
    pub subscription_id: SubscriptionId,
    pub subscription_period_id: SubscriptionPeriodId,
    pub trial: bool,
    pub user_count: i32,
}
