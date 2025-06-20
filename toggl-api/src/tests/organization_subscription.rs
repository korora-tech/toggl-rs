use crate::models::api::ids::{OrganizationId, PricingPlanId};
use crate::models::api::organization_subscription::*;
use crate::tests::with_mockito;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

#[test]
fn test_get_subscription() -> Result<()> {
    let response = json!({
        "active_users": 5,
        "auto_renew": true,
        "billing_period_in_months": 12,
        "currency": "USD",
        "enterprise": false,
        "is_subscription_beta": false,
        "is_unified": true,
        "plan_name": "Pro",
        "pricing_plan_id": 2,
        "seat_cost_in_cents": 1000,
        "seats": 10,
        "state": "active",
        "trial_available": false
    });

    with_mockito(
        Method::GET,
        "/organizations/123/subscription",
        200,
        Some(response),
        |client| {
            let subscription = client
                .organizations()
                .get_subscription(OrganizationId(123))?;
            assert_eq!(subscription.active_users, 5);
            assert!(subscription.auto_renew);
            assert_eq!(subscription.plan_name, "Pro");
            Ok(())
        },
    )
}

#[test]
fn test_create_subscription() -> Result<()> {
    let create_subscription = CreateOrganizationSubscription {
        pricing_plan_tag: "pro".to_string(),
    };

    let response = json!({
        "active_users": 1,
        "auto_renew": true,
        "billing_period_in_months": 12,
        "currency": "USD",
        "enterprise": false,
        "is_subscription_beta": false,
        "is_unified": true,
        "plan_name": "Pro",
        "pricing_plan_id": 2,
        "seat_cost_in_cents": 1000,
        "seats": 10,
        "state": "active",
        "trial_available": false
    });

    with_mockito(
        Method::POST,
        "/organizations/123/subscription",
        200,
        Some(response),
        |client| {
            let subscription = client
                .organizations()
                .create_subscription(OrganizationId(123), &create_subscription)?;
            assert_eq!(subscription.plan_name, "Pro");
            Ok(())
        },
    )
}

#[test]
fn test_update_subscription() -> Result<()> {
    let update_subscription = UpdateOrganizationSubscription {
        pricing_plan_tag: "enterprise".to_string(),
    };

    let response = json!({
        "active_users": 10,
        "auto_renew": true,
        "billing_period_in_months": 12,
        "currency": "USD",
        "enterprise": true,
        "is_subscription_beta": false,
        "is_unified": true,
        "plan_name": "Enterprise",
        "pricing_plan_id": 3,
        "seat_cost_in_cents": 2000,
        "seats": 50,
        "state": "active",
        "trial_available": false
    });

    with_mockito(
        Method::PUT,
        "/organizations/123/subscription",
        200,
        Some(response),
        |client| {
            let subscription = client
                .organizations()
                .update_subscription(OrganizationId(123), &update_subscription)?;
            assert_eq!(subscription.plan_name, "Enterprise");
            assert!(subscription.enterprise);
            Ok(())
        },
    )
}

#[test]
fn test_cancel_subscription() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/organizations/123/subscription",
        204,
        None,
        |client| {
            client
                .organizations()
                .cancel_subscription(OrganizationId(123))?;
            Ok(())
        },
    )
}

#[test]
fn test_submit_cancellation_feedback() -> Result<()> {
    let feedback = CancellationFeedback {
        responses_submitted: vec![FeedbackResponse {
            reason: "Too expensive".to_string(),
            details: Some("We found a cheaper alternative".to_string()),
        }],
    };

    with_mockito(
        Method::POST,
        "/organizations/123/subscription/cancellation_feedback",
        204,
        None,
        |client| {
            client
                .organizations()
                .submit_cancellation_feedback(OrganizationId(123), &feedback)?;
            Ok(())
        },
    )
}

#[test]
fn test_get_subscription_customer() -> Result<()> {
    let response = json!({
        "id": 456,
        "email": "billing@company.com",
        "name": "John Doe",
        "company_name": "Acme Corp",
        "country": "US",
        "created_at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::GET,
        "/organizations/123/subscription/customer",
        200,
        Some(response),
        |client| {
            let customer = client
                .organizations()
                .get_subscription_customer(OrganizationId(123))?;
            assert_eq!(customer.email, "billing@company.com");
            assert_eq!(customer.name, Some("John Doe".to_string()));
            Ok(())
        },
    )
}

#[test]
fn test_create_subscription_customer() -> Result<()> {
    let create_customer = CreateCustomer {
        email: "new@company.com".to_string(),
        name: Some("Jane Smith".to_string()),
        company_name: Some("New Corp".to_string()),
        company_address: None,
        company_vat_number: None,
        country: Some("US".to_string()),
    };

    let response = json!({
        "id": 789,
        "email": "new@company.com",
        "name": "Jane Smith",
        "company_name": "New Corp",
        "country": "US",
        "created_at": "2023-01-02T00:00:00Z"
    });

    with_mockito(
        Method::POST,
        "/organizations/123/subscription/customer",
        200,
        Some(response),
        |client| {
            let customer = client
                .organizations()
                .create_subscription_customer(OrganizationId(123), &create_customer)?;
            assert_eq!(customer.email, "new@company.com");
            Ok(())
        },
    )
}

#[test]
fn test_update_subscription_customer() -> Result<()> {
    let update_customer = UpdateCustomer {
        email: Some("updated@company.com".to_string()),
        name: None,
        company_name: None,
        company_address: None,
        company_vat_number: None,
        country: None,
    };

    let response = json!({
        "id": 456,
        "email": "updated@company.com",
        "name": "John Doe",
        "company_name": "Acme Corp",
        "country": "US",
        "created_at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::PUT,
        "/organizations/123/subscription/customer",
        200,
        Some(response),
        |client| {
            let customer = client
                .organizations()
                .update_subscription_customer(OrganizationId(123), &update_customer)?;
            assert_eq!(customer.email, "updated@company.com");
            Ok(())
        },
    )
}

#[test]
fn test_request_discount() -> Result<()> {
    let discount_request = DiscountRequest {
        percentage: 20,
        duration_months: 6,
        reason: "Long-term customer".to_string(),
    };

    let response = json!({
        "success": true
    });

    with_mockito(
        Method::POST,
        "/organizations/123/subscription/discount_request",
        200,
        Some(response),
        |client| {
            let result = client
                .organizations()
                .request_discount(OrganizationId(123), &discount_request)?;
            assert!(result.success);
            Ok(())
        },
    )
}

#[test]
fn test_feature_upsell_multi() -> Result<()> {
    let upsell_request = FeatureUpsellRequest {
        features: vec!["time_tracking".to_string(), "reporting".to_string()],
    };

    let response = json!({
        "success": true
    });

    with_mockito(
        Method::POST,
        "/organizations/123/subscription/feature_upsell_multi",
        200,
        Some(response),
        |client| {
            let result = client
                .organizations()
                .feature_upsell_multi(OrganizationId(123), &upsell_request)?;
            assert!(result.success);
            Ok(())
        },
    )
}

#[test]
fn test_get_invoice_summary() -> Result<()> {
    let response = json!({
        "amount_in_cents": 10000,
        "currency": "USD",
        "items": [
            {
                "description": "Pro plan - 10 users",
                "amount_in_cents": 10000,
                "quantity": 1
            }
        ]
    });

    with_mockito(
        Method::GET,
        "/organizations/123/subscription/invoice_summary",
        200,
        Some(response),
        |client| {
            let summary = client
                .organizations()
                .get_invoice_summary(OrganizationId(123))?;
            assert_eq!(summary.amount_in_cents, 10000);
            assert_eq!(summary.currency, "USD");
            assert_eq!(summary.items.len(), 1);
            Ok(())
        },
    )
}

#[test]
fn test_get_payment_failed() -> Result<()> {
    let response = json!({
        "reason": "Card declined",
        "failed_at": "2023-01-01T00:00:00Z",
        "retry_at": "2023-01-02T00:00:00Z"
    });

    with_mockito(
        Method::GET,
        "/organizations/123/subscription/payment_failed",
        200,
        Some(response),
        |client| {
            let payment_failed = client
                .organizations()
                .get_payment_failed(OrganizationId(123))?;
            assert_eq!(payment_failed.reason, "Card declined");
            Ok(())
        },
    )
}

#[test]
fn test_apply_promo_code() -> Result<()> {
    let promo_code = OrganizationPromoCode {
        code: "SAVE20".to_string(),
    };

    let response = json!({
        "success": true
    });

    with_mockito(
        Method::POST,
        "/organizations/123/subscription/promocode",
        200,
        Some(response),
        |client| {
            let result = client
                .organizations()
                .apply_promo_code(OrganizationId(123), &promo_code)?;
            assert!(result.success);
            Ok(())
        },
    )
}

#[test]
fn test_remove_promo_code() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/organizations/123/subscription/promocode",
        204,
        None,
        |client| {
            client
                .organizations()
                .remove_promo_code(OrganizationId(123))?;
            Ok(())
        },
    )
}

#[test]
fn test_apply_referral_bonus() -> Result<()> {
    let referral = ReferralBonus {
        referral_code: "REF123".to_string(),
    };

    let response = json!({
        "success": true
    });

    with_mockito(
        Method::POST,
        "/organizations/123/subscription/referral_bonus",
        200,
        Some(response),
        |client| {
            let result = client
                .organizations()
                .apply_referral_bonus(OrganizationId(123), &referral)?;
            assert!(result.success);
            Ok(())
        },
    )
}

#[test]
fn test_create_setup_intent() -> Result<()> {
    let response = json!({
        "client_secret": "pi_1234567890_secret",
        "payment_method_types": ["card"]
    });

    with_mockito(
        Method::POST,
        "/organizations/123/subscription/setup_intent",
        200,
        Some(response),
        |client| {
            let intent = client
                .organizations()
                .create_setup_intent(OrganizationId(123))?;
            assert_eq!(intent.client_secret, "pi_1234567890_secret");
            assert_eq!(intent.payment_method_types, vec!["card"]);
            Ok(())
        },
    )
}

#[test]
fn test_start_trial() -> Result<()> {
    let trial = StartTrial {
        pricing_plan_id: Some(PricingPlanId(2)),
    };

    let response = json!({
        "active_users": 1,
        "auto_renew": true,
        "billing_period_in_months": 12,
        "currency": "USD",
        "enterprise": false,
        "is_subscription_beta": false,
        "is_unified": true,
        "plan_name": "Pro",
        "pricing_plan_id": 2,
        "seat_cost_in_cents": 1000,
        "seats": 10,
        "state": "trial",
        "trial_available": false,
        "trial_end_date": "2023-02-01T00:00:00Z"
    });

    with_mockito(
        Method::POST,
        "/organizations/123/subscription/trial",
        200,
        Some(response),
        |client| {
            let subscription = client
                .organizations()
                .start_trial(OrganizationId(123), &trial)?;
            assert_eq!(subscription.state, "trial");
            Ok(())
        },
    )
}

#[test]
fn test_request_upgrade() -> Result<()> {
    let upgrade_request = UpgradeRequest {
        user_count: Some(20),
        message: Some("Need more users".to_string()),
    };

    let response = json!({
        "success": true
    });

    with_mockito(
        Method::POST,
        "/organizations/123/subscription/upgrade_request/advanced_reporting",
        200,
        Some(response),
        |client| {
            let result = client.organizations().request_upgrade(
                OrganizationId(123),
                "advanced_reporting",
                &upgrade_request,
            )?;
            assert!(result.success);
            Ok(())
        },
    )
}

#[test]
fn test_get_payment_records() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "amount": 100.00,
            "currency": "USD",
            "created_at": "2023-01-01T00:00:00Z",
            "status": "paid",
            "invoice_id": 123
        }
    ]);

    with_mockito(
        Method::GET,
        "/organizations/123/payment_records",
        200,
        Some(response),
        |client| {
            let records = client
                .organizations()
                .get_payment_records(OrganizationId(123), None)?;
            assert_eq!(records.len(), 1);
            assert_eq!(records[0].amount, 100.00);
            assert_eq!(records[0].status, "paid");
            Ok(())
        },
    )
}

#[test]
fn test_get_payment_records_with_unified() -> Result<()> {
    let response = json!([
        {
            "id": 2,
            "amount": 200.00,
            "currency": "USD",
            "created_at": "2023-01-01T00:00:00Z",
            "status": "paid",
            "invoice_id": 456
        }
    ]);

    with_mockito(
        Method::GET,
        "/organizations/123/payment_records?is_unified=true",
        200,
        Some(response),
        |client| {
            let records = client
                .organizations()
                .get_payment_records(OrganizationId(123), Some(true))?;
            assert_eq!(records.len(), 1);
            assert_eq!(records[0].amount, 200.00);
            Ok(())
        },
    )
}

#[test]
fn test_get_plans() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "name": "Free",
            "tag": "free",
            "currency": "USD",
            "amount_in_cents": 0,
            "max_users": 5,
            "features": ["basic_tracking"]
        },
        {
            "id": 2,
            "name": "Pro",
            "tag": "pro",
            "currency": "USD",
            "amount_in_cents": 1000,
            "max_users": null,
            "features": ["basic_tracking", "reporting", "integrations"]
        }
    ]);

    with_mockito(
        Method::GET,
        "/organizations/123/plans",
        200,
        Some(response),
        |client| {
            let plans = client.organizations().get_plans(OrganizationId(123))?;
            assert_eq!(plans.len(), 2);
            assert_eq!(plans[0].name, "Free");
            assert_eq!(plans[1].name, "Pro");
            Ok(())
        },
    )
}

#[test]
fn test_get_plan() -> Result<()> {
    let response = json!({
        "id": 2,
        "name": "Pro",
        "tag": "pro",
        "currency": "USD",
        "amount_in_cents": 1000,
        "max_users": null,
        "features": ["basic_tracking", "reporting", "integrations"]
    });

    with_mockito(
        Method::GET,
        "/organizations/123/plans/2",
        200,
        Some(response),
        |client| {
            let plan = client
                .organizations()
                .get_plan(OrganizationId(123), PricingPlanId(2))?;
            assert_eq!(plan.name, "Pro");
            assert_eq!(plan.amount_in_cents, 1000);
            Ok(())
        },
    )
}
