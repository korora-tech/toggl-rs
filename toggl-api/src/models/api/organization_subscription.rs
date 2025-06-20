use serde::{Deserialize, Serialize};

use super::ids::{
    CompanyId, CurrencyCode, CustomerId, InvoiceInfoId, PaymentRecordId, PricingPlanId,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationSubscription {
    pub active_users: u32,
    pub auto_renew: bool,
    pub billing_period_in_months: u32,
    pub campaign_available: Option<bool>,
    pub cancel_date: Option<String>,
    pub card_details: Option<CardDetails>,
    pub company_id: Option<CompanyId>,
    pub contact_details: Option<ContactDetail>,
    pub currency: String,
    pub current_period_ends_at: Option<String>,
    pub current_period_starts_at: Option<String>,
    pub customer_id: Option<CustomerId>,
    pub end_date: Option<String>,
    pub enterprise: bool,
    pub is_subscription_beta: bool,
    pub is_unified: bool,
    pub last_invoice: Option<InvoiceInfo>,
    pub last_payment: Option<PaymentInfo>,
    pub last_pricing_plan_id: Option<PricingPlanId>,
    pub new_signup_trial: Option<bool>,
    pub next_payment_date: Option<String>,
    pub payment_failed: Option<bool>,
    pub payment_method: Option<String>,
    pub plan_name: String,
    pub pricing_plan_id: PricingPlanId,
    pub renewal_at: Option<String>,
    pub renewal_date: Option<String>,
    pub seat_cost_in_cents: i64,
    pub seats: u32,
    pub site: Option<String>,
    pub start_date: Option<String>,
    pub state: String,
    pub subscription_created_at: Option<String>,
    pub subscription_period: Option<SubscriptionPeriod>,
    pub trial_available: bool,
    pub trial_end_date: Option<String>,
    pub trial_start_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardDetails {
    pub card_number: String,
    pub card_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContactDetail {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceInfo {
    pub amount: f64,
    pub created_at: String,
    pub currency_id: CurrencyCode,
    pub due: Option<f64>,
    pub id: InvoiceInfoId,
    pub paid_at: Option<String>,
    pub tax_percentage: Option<f64>,
    pub total_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentInfo {
    pub amount: f64,
    pub created_at: String,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionPeriod {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrganizationSubscription {
    pub pricing_plan_tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOrganizationSubscription {
    pub pricing_plan_tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancellationFeedback {
    pub responses_submitted: Vec<FeedbackResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedbackResponse {
    pub reason: String,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub id: CustomerId,
    pub email: String,
    pub name: Option<String>,
    pub company_name: Option<String>,
    pub company_address: Option<String>,
    pub company_vat_number: Option<String>,
    pub country: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCustomer {
    pub email: String,
    pub name: Option<String>,
    pub company_name: Option<String>,
    pub company_address: Option<String>,
    pub company_vat_number: Option<String>,
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCustomer {
    pub email: Option<String>,
    pub name: Option<String>,
    pub company_name: Option<String>,
    pub company_address: Option<String>,
    pub company_vat_number: Option<String>,
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscountRequest {
    pub percentage: u32,
    pub duration_months: u32,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeatureUpsellRequest {
    pub features: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceSummary {
    pub amount_in_cents: i64,
    pub currency: String,
    pub items: Vec<InvoiceItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceItem {
    pub description: String,
    pub amount_in_cents: i64,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentFailed {
    pub reason: String,
    pub failed_at: String,
    pub retry_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationPromoCode {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferralBonus {
    pub referral_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupIntent {
    pub client_secret: String,
    pub payment_method_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartTrial {
    pub pricing_plan_id: Option<PricingPlanId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpgradeRequest {
    pub user_count: Option<u32>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentRecord {
    pub id: PaymentRecordId,
    pub amount: f64,
    pub currency: String,
    pub created_at: String,
    pub status: String,
    pub invoice_id: Option<InvoiceInfoId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationPlan {
    pub id: PricingPlanId,
    pub name: String,
    pub tag: String,
    pub currency: String,
    pub amount_in_cents: i64,
    pub max_users: Option<u32>,
    pub features: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessResponse {
    pub success: bool,
}
