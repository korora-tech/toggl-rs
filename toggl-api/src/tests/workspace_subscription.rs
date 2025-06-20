use serde_json::json;
use toggl_core::Result;
use toggl_core::WorkspaceId;

use super::with_mockito;
use reqwest::Method;

#[test]
fn test_get_workspace_subscription() -> Result<()> {
    let response = json!({
        "active_users": 5,
        "auto_renew": true,
        "billing_period_in_months": 12,
        "campaign_available": false,
        "cancel_date": null,
        "card_details": {
            "added_at": "2024-01-01T00:00:00Z",
            "card_number": "****1234",
            "card_type": "Visa",
            "creator_id": 123,
            "creator_name": "John Doe",
            "expiry_date": "12/25",
            "holder_name": "John Doe"
        },
        "company_id": 456,
        "contact_details": {
            "company_address": "123 Main St",
            "company_city": "San Francisco",
            "company_name": "Acme Corp",
            "contact_detail_id": 789,
            "contact_email": "billing@acme.com",
            "contact_person": "Jane Doe",
            "country_id": 1,
            "country_subdivision_id": null,
            "created_at": "2024-01-01T00:00:00Z",
            "customer_id": 456,
            "is_eu_resident": false,
            "updated_at": "2024-01-15T00:00:00Z",
            "user_id": 123,
            "vat_number": null,
            "vat_number_valid": null,
            "vat_number_validated_at": null
        },
        "currency": "USD",
        "current_period_ends_at": "2025-01-01T00:00:00Z",
        "current_period_starts_at": "2024-01-01T00:00:00Z",
        "customer_id": 456,
        "end_date": null,
        "enterprise": false,
        "is_subscription_beta": false,
        "is_unified": false,
        "last_invoice": {
            "amount": 10000,
            "created_at": "2024-01-01T00:00:00Z",
            "currency_id": 1,
            "due": "2024-01-15T00:00:00Z",
            "id": 999,
            "paid_at": "2024-01-10T00:00:00Z",
            "tax_percentage": 0.0,
            "total_amount": 10000
        },
        "last_payment": {
            "created_at": "2024-01-10T00:00:00Z",
            "description": "Subscription payment",
            "id": 888,
            "status": "success"
        },
        "last_pricing_plan_id": 2,
        "new_signup_trial": false,
        "next_payment_date": "2025-01-01T00:00:00Z",
        "payment_failed": false,
        "payment_method": "card",
        "plan_name": "Pro",
        "pricing_plan_id": 3,
        "renewal_at": "2025-01-01T00:00:00Z",
        "renewal_date": "2025-01-01T00:00:00Z",
        "seat_cost_in_cents": 2000,
        "seats": 5,
        "site": "toggl.com",
        "start_date": null,
        "state": "active",
        "subscription_created_at": "2024-01-01T00:00:00Z",
        "subscription_period": {
            "created_at": "2024-01-01T00:00:00Z",
            "finished_on": null,
            "started_on": "2024-01-01T00:00:00Z",
            "subscription_id": 777,
            "subscription_period_id": 666,
            "trial": false,
            "user_count": 5
        },
        "trial_available": false,
        "trial_end_date": null,
        "trial_start_date": null
    });

    with_mockito(
        Method::GET,
        "/workspaces/111/subscription",
        200,
        Some(response),
        |client| {
            let subscription = client.workspaces().get_subscription(WorkspaceId(111))?;
            assert_eq!(subscription.active_users, 5);
            assert!(subscription.auto_renew);
            assert_eq!(subscription.billing_period_in_months, 12);
            assert_eq!(subscription.currency, "USD");
            assert_eq!(subscription.plan_name, "Pro");
            assert_eq!(subscription.seats, 5);
            assert_eq!(subscription.seat_cost_in_cents, 2000);
            assert_eq!(subscription.state, "active");
            assert_eq!(subscription.payment_method, "card");
            assert!(!subscription.payment_failed);

            // Check card details
            let card = subscription.card_details.unwrap();
            assert_eq!(card.card_number, "****1234");
            assert_eq!(card.card_type, "Visa");
            assert_eq!(card.holder_name, "John Doe");

            // Check contact details
            let contact = subscription.contact_details.unwrap();
            assert_eq!(contact.company_name.unwrap(), "Acme Corp");
            assert_eq!(contact.contact_email, "billing@acme.com");

            // Check last invoice
            let invoice = subscription.last_invoice.unwrap();
            assert_eq!(invoice.amount, 10000);
            assert_eq!(invoice.total_amount, 10000);

            // Check last payment
            let payment = subscription.last_payment.unwrap();
            assert_eq!(payment.status, "success");

            // Check subscription period
            let period = subscription.subscription_period.unwrap();
            assert_eq!(period.user_count, 5);
            assert!(!period.trial);

            Ok(())
        },
    )
}
