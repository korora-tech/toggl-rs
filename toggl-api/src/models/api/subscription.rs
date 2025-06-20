use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use toggl_core::{
    CustomerId, InvoiceItemId, OrganizationId, PaymentMethodId, PricingPlanId, SubscriptionId,
    WorkspaceId,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub workspace_id: Option<WorkspaceId>,
    pub organization_id: Option<OrganizationId>,
    pub customer_id: CustomerId,
    pub pricing_plan_id: PricingPlanId,
    pub renewal_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub currency: String,
    pub auto_renew: bool,
    pub payment_details: Option<PaymentDetails>,
    pub period: SubscriptionPeriod,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionPeriod {
    pub current_period_ends_at: Option<DateTime<Utc>>,
    pub current_period_starts_at: Option<DateTime<Utc>>,
    pub trial: bool,
    pub trial_ends_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentDetails {
    pub payment_method_id: Option<PaymentMethodId>,
    pub next_payment_date: Option<DateTime<Utc>>,
    pub next_payment_amount: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSubscription {
    pub workspace_id: Option<WorkspaceId>,
    pub organization_id: Option<OrganizationId>,
    pub pricing_plan_id: PricingPlanId,
    pub payment_method_id: Option<PaymentMethodId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSubscription {
    pub pricing_plan_id: Option<PricingPlanId>,
    pub payment_method_id: Option<PaymentMethodId>,
    pub auto_renew: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invoice {
    pub id: InvoiceItemId,
    pub subscription_id: SubscriptionId,
    pub customer_id: CustomerId,
    pub currency: String,
    pub amount: f64,
    pub status: String,
    pub number: String,
    pub issued_at: DateTime<Utc>,
    pub due_date: NaiveDate,
    pub paid_at: Option<DateTime<Utc>>,
    pub pdf_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCode {
    pub code: String,
    pub description: String,
    pub percentage: Option<f64>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub duration: String,
    pub duration_in_months: Option<u32>,
    pub valid: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplyCoupon {
    pub coupon_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub plan_id: String,
    pub name: String,
    pub pricing_plan_id: PricingPlanId,
    pub price: f64,
    pub currency: String,
    pub workspace_limit: Option<u64>,
    pub user_limit: Option<u64>,
    pub features: Vec<String>,
}
