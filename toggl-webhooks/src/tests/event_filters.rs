//! Tests for event filters endpoint

use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

use super::with_mockito;

#[test]
fn test_get_event_filters() -> Result<()> {
    let response = json!({
        "event_filters": [
            {
                "name": "time_entry",
                "actions": ["created", "updated", "deleted"]
            },
            {
                "name": "project",
                "actions": ["created", "updated", "deleted"]
            },
            {
                "name": "client",
                "actions": ["created", "updated", "deleted"]
            },
            {
                "name": "tag",
                "actions": ["created", "updated", "deleted"]
            },
            {
                "name": "task",
                "actions": ["created", "updated", "deleted"]
            },
            {
                "name": "user",
                "actions": ["created", "updated", "deleted"]
            },
            {
                "name": "workspace_user",
                "actions": ["created", "updated", "deleted"]
            },
            {
                "name": "group",
                "actions": ["created", "updated", "deleted"]
            }
        ]
    });

    with_mockito(
        Method::GET,
        "/api/v9/event_filters",
        200,
        Some(response),
        |client| {
            let filters = client.get_event_filters()?;
            assert!(filters.event_filters.is_some());

            let event_filters = filters.event_filters.unwrap();
            assert_eq!(event_filters.len(), 8);

            // Check first filter
            assert_eq!(event_filters[0].name, Some("time_entry".to_string()));
            assert!(event_filters[0].actions.is_some());
            let actions = event_filters[0].actions.as_ref().unwrap();
            assert_eq!(actions.len(), 3);
            assert!(actions.contains(&"created".to_string()));
            assert!(actions.contains(&"updated".to_string()));
            assert!(actions.contains(&"deleted".to_string()));

            Ok(())
        },
    )
}

#[test]
fn test_get_event_filters_empty() -> Result<()> {
    let response = json!({
        "event_filters": []
    });

    with_mockito(
        Method::GET,
        "/api/v9/event_filters",
        200,
        Some(response),
        |client| {
            let filters = client.get_event_filters()?;
            assert!(filters.event_filters.is_some());
            assert_eq!(filters.event_filters.unwrap().len(), 0);
            Ok(())
        },
    )
}
