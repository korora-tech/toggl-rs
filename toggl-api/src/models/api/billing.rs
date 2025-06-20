use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::ids::{FeatureId, PaymentMethodId, PricingPlanId, ProductId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plan {
    pub id: PricingPlanId,
    pub name: String,
    pub pricing_plan_id: PricingPlanId,
    pub monthly_price_in_cents: u64,
    pub yearly_price_in_cents: u64,
    pub max_workspaces: u32,
    pub max_workspace_users: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlanFeature {
    pub feature_id: FeatureId,
    pub name: String,
    pub plan_id: PricingPlanId,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FancyPlan {
    pub product_id: ProductId,
    pub name: String,
    pub localized_name: Option<String>,
    pub actual_price: f64,
    pub price_before_discount: Option<f64>,
    pub discount_percentage: Option<f64>,
    pub currency: String,
    pub period: String,
    pub fixed: bool,
    pub user_count: Option<u32>,
    pub max_workspace_users: Option<u32>,
    pub trial_days: Option<u32>,
    pub description: Option<String>,
    pub features: Vec<FancyPlanFeature>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FancyPlanFeature {
    pub name: String,
    pub localized_name: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PricingPlans {
    pub free_plan: FancyPlan,
    pub personal_plans: Vec<FancyPlan>,
    pub team_plans: Vec<FancyPlan>,
    pub custom_plan: Option<FancyPlan>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethod {
    pub id: PaymentMethodId,
    pub type_: String,
    pub card: Option<PaymentCard>,
    pub sepa_debit: Option<SEPADebit>,
    pub us_bank_account: Option<USBankAccount>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentCard {
    pub brand: String,
    pub last4: String,
    pub exp_month: u8,
    pub exp_year: u16,
    pub holder_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SEPADebit {
    pub last4: String,
    pub country: String,
    pub bank_code: Option<String>,
    pub fingerprint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct USBankAccount {
    pub last4: String,
    pub bank_name: String,
    pub account_holder_type: String,
    pub account_type: String,
    pub fingerprint: String,
}
