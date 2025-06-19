use crate::error::Result;
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;

use super::with_mockito;

#[test]
fn test_get_subscription_plans() -> Result<()> {
    let response = json!([
        {
            "plan_id": "free",
            "name": "Free",
            "pricing_plan_id": 1,
            "price": 0.0,
            "currency": "USD",
            "workspace_limit": 5,
            "user_limit": 5,
            "features": ["basic_time_tracking", "reporting"]
        },
        {
            "plan_id": "starter",
            "name": "Starter",
            "pricing_plan_id": 2,
            "price": 10.0,
            "currency": "USD",
            "workspace_limit": null,
            "user_limit": null,
            "features": ["basic_time_tracking", "reporting", "billable_rates", "time_rounding", "saved_reports"]
        },
        {
            "plan_id": "premium",
            "name": "Premium",
            "pricing_plan_id": 3,
            "price": 20.0,
            "currency": "USD",
            "workspace_limit": null,
            "user_limit": null,
            "features": ["basic_time_tracking", "reporting", "billable_rates", "time_rounding", "saved_reports", "time_tracking_reminders", "project_time_estimates", "tasks", "project_dashboard"]
        }
    ]);

    with_mockito(
        Method::GET,
        "/subscriptions/plans",
        200,
        Some(response),
        |client| {
            let plans = client.subscriptions().plans()?;
            assert_eq!(3, plans.len());
            assert_eq!("free", plans[0].plan_id);
            assert_eq!(0.0, plans[0].price);
            assert_eq!(Some(5), plans[0].workspace_limit);
            assert_eq!("starter", plans[1].plan_id);
            assert_eq!(10.0, plans[1].price);
            assert!(plans[1].workspace_limit.is_none());
            assert_eq!("premium", plans[2].plan_id);
            assert_eq!(20.0, plans[2].price);
            assert_eq!(9, plans[2].features.len());
            Ok(())
        },
    )
}
