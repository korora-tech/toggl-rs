//! Tests for subscription endpoints

use reqwest::Method;
use serde_json::json;
use toggl_core::{Result, WorkspaceId};

use crate::models::*;

use super::{with_mockito, with_mockito_params};

#[test]
fn test_get_subscriptions() -> Result<()> {
    let workspace_id = WorkspaceId::new(12345);
    let response = json!([
        {
            "subscription_id": "sub_123",
            "workspace_id": 12345,
            "url_callback": "https://example.com/webhook",
            "enabled": true,
            "description": "Test subscription",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "event_filters": [
                {
                    "entity": "time_entry",
                    "action": "created"
                }
            ]
        }
    ]);

    with_mockito_params(
        Method::GET,
        "/api/v9/subscriptions",
        vec![("workspace_id", "12345")],
        200,
        Some(response),
        |client| {
            let subscriptions = client.get_subscriptions(workspace_id)?;
            assert_eq!(subscriptions.len(), 1);
            assert_eq!(subscriptions[0].subscription_id, "sub_123");
            assert_eq!(
                subscriptions[0].url_callback,
                Some("https://example.com/webhook".to_string())
            );
            assert_eq!(subscriptions[0].enabled, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_create_subscription() -> Result<()> {
    let create_request = CreateSubscription {
        workspace_id: WorkspaceId::new(12345),
        url_callback: "https://example.com/webhook".to_string(),
        enabled: Some(true),
        description: Some("Test subscription".to_string()),
        event_filters: Some(vec![SubscriptionInEventFilter {
            entity: "time_entry".to_string(),
            action: "created".to_string(),
        }]),
        secret: Some("my-secret-key".to_string()),
        user_agent: Some("MyApp/1.0".to_string()),
        has_pending_events: None,
    };

    let response = json!({
        "subscription_id": "sub_123",
        "workspace_id": 12345,
        "url_callback": "https://example.com/webhook",
        "enabled": true,
        "description": "Test subscription",
        "created_at": "2024-01-01T00:00:00Z",
        "event_filters": [
            {
                "entity": "time_entry",
                "action": "created"
            }
        ],
        "secret": "my-secret-key",
        "user_agent": "MyApp/1.0"
    });

    with_mockito(
        Method::POST,
        "/api/v9/subscriptions",
        201,
        Some(response),
        |client| {
            let subscription = client.create_subscription(&create_request)?;
            assert_eq!(subscription.subscription_id, "sub_123");
            assert_eq!(subscription.workspace_id, Some(WorkspaceId::new(12345)));
            assert_eq!(subscription.enabled, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_get_subscription() -> Result<()> {
    let subscription_id = "sub_123";
    let response = json!({
        "subscription_id": "sub_123",
        "workspace_id": 12345,
        "url_callback": "https://example.com/webhook",
        "enabled": true,
        "description": "Test subscription",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z",
        "validated_at": "2024-01-01T00:00:00Z",
        "event_filters": [
            {
                "entity": "time_entry",
                "action": "created"
            }
        ]
    });

    with_mockito(
        Method::GET,
        "/api/v9/subscriptions/sub_123",
        200,
        Some(response),
        |client| {
            let subscription = client.get_subscription(subscription_id)?;
            assert_eq!(subscription.subscription_id, "sub_123");
            assert_eq!(subscription.workspace_id, Some(WorkspaceId::new(12345)));
            assert!(subscription.validated_at.is_some());
            Ok(())
        },
    )
}

#[test]
fn test_update_subscription() -> Result<()> {
    let subscription_id = "sub_123";
    let update_request = UpdateSubscription {
        enabled: Some(false),
        description: Some("Updated subscription".to_string()),
        url_callback: Some("https://example.com/webhook-updated".to_string()),
        event_filters: Some(vec![SubscriptionUpdateEventFilter {
            entity: Some("project".to_string()),
            action: Some("updated".to_string()),
        }]),
        secret: None,
        user_agent: None,
        has_pending_events: None,
    };

    let response = json!({
        "subscription_id": "sub_123",
        "workspace_id": 12345,
        "url_callback": "https://example.com/webhook-updated",
        "enabled": false,
        "description": "Updated subscription",
        "updated_at": "2024-01-02T00:00:00Z",
        "event_filters": [
            {
                "entity": "project",
                "action": "updated"
            }
        ]
    });

    with_mockito(
        Method::PUT,
        "/api/v9/subscriptions/sub_123",
        200,
        Some(response),
        |client| {
            let subscription = client.update_subscription(subscription_id, &update_request)?;
            assert_eq!(subscription.subscription_id, "sub_123");
            assert_eq!(subscription.enabled, Some(false));
            assert_eq!(
                subscription.description,
                Some("Updated subscription".to_string())
            );
            Ok(())
        },
    )
}

#[test]
fn test_delete_subscription() -> Result<()> {
    let subscription_id = "sub_123";

    with_mockito(
        Method::DELETE,
        "/api/v9/subscriptions/sub_123",
        204,
        None,
        |client| {
            client.delete_subscription(subscription_id)?;
            Ok(())
        },
    )
}

#[test]
fn test_subscription_error_handling() -> Result<()> {
    let subscription_id = "sub_not_found";
    let error_response = json!({
        "code": 404,
        "message": "Subscription not found",
        "tip": "Check the subscription ID"
    });

    with_mockito(
        Method::GET,
        "/api/v9/subscriptions/sub_not_found",
        404,
        Some(error_response),
        |client| match client.get_subscription(subscription_id) {
            Err(e) => {
                assert!(e.to_string().contains("Subscription not found"));
                Ok(())
            }
            Ok(_) => panic!("Expected error but got success"),
        },
    )
}

#[test]
fn test_create_subscription_with_wildcards() -> Result<()> {
    let request = CreateSubscription {
        workspace_id: WorkspaceId::new(12345),
        url_callback: "https://example.com/webhook".to_string(),
        enabled: Some(true),
        description: Some("Wildcard subscription".to_string()),
        event_filters: Some(vec![
            SubscriptionInEventFilter {
                entity: "*".to_string(), // All entities
                action: "created".to_string(),
            },
            SubscriptionInEventFilter {
                entity: "project".to_string(),
                action: "*".to_string(), // All actions
            },
            SubscriptionInEventFilter {
                entity: "*".to_string(), // All entities
                action: "*".to_string(), // All actions
            },
        ]),
        secret: Some("test-secret".to_string()),
        user_agent: None,
        has_pending_events: None,
    };

    let response = json!({
        "subscription_id": "sub_wildcard",
        "workspace_id": 12345,
        "url_callback": "https://example.com/webhook",
        "enabled": true,
        "description": "Wildcard subscription",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z",
        "event_filters": [
            {
                "entity": "*",
                "action": "created"
            },
            {
                "entity": "project",
                "action": "*"
            },
            {
                "entity": "*",
                "action": "*"
            }
        ]
    });

    with_mockito(
        Method::POST,
        "/api/v9/subscriptions",
        201,
        Some(response),
        |client| {
            let subscription = client.create_subscription(&request)?;
            assert_eq!(subscription.subscription_id, "sub_wildcard");

            // Verify wildcard filters were accepted
            if let Some(filters) = subscription.event_filters {
                assert_eq!(filters.len(), 3);

                // Check first filter: all entities, created action
                assert_eq!(filters[0].entity, Some("*".to_string()));
                assert_eq!(filters[0].action, Some("created".to_string()));

                // Check second filter: project entity, all actions
                assert_eq!(filters[1].entity, Some("project".to_string()));
                assert_eq!(filters[1].action, Some("*".to_string()));

                // Check third filter: all entities, all actions
                assert_eq!(filters[2].entity, Some("*".to_string()));
                assert_eq!(filters[2].action, Some("*".to_string()));
            } else {
                panic!("Expected event filters in response");
            }

            Ok(())
        },
    )
}
