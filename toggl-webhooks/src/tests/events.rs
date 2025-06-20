//! Tests for event handling and signature verification

use crate::events::verify_signature;

#[test]
fn test_verify_signature_valid() {
    let payload = b"test payload";
    let secret = "my-secret-key";

    // Generate the expected signature
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let result = mac.finalize();
    let expected_signature = hex::encode(result.into_bytes());

    // Verify it works
    assert!(verify_signature(payload, &expected_signature, secret));
}

#[test]
fn test_verify_signature_invalid() {
    let payload = b"test payload";
    let secret = "my-secret-key";
    let invalid_signature = "invalid_signature_12345";

    assert!(!verify_signature(payload, invalid_signature, secret));
}

#[test]
fn test_verify_signature_wrong_secret() {
    let payload = b"test payload";
    let secret1 = "my-secret-key";
    let secret2 = "different-secret";

    // Generate signature with secret1
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret1.as_bytes()).unwrap();
    mac.update(payload);
    let result = mac.finalize();
    let signature = hex::encode(result.into_bytes());

    // Try to verify with secret2
    assert!(!verify_signature(payload, &signature, secret2));
}

#[test]
fn test_verify_signature_empty_payload() {
    let payload = b"";
    let secret = "my-secret-key";

    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let result = mac.finalize();
    let expected_signature = hex::encode(result.into_bytes());

    assert!(verify_signature(payload, &expected_signature, secret));
}

#[test]
fn test_verify_signature_with_sha256_prefix() {
    let payload = b"test webhook payload";
    let secret = "my-secret-key";

    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let result = mac.finalize();
    let hash = hex::encode(result.into_bytes());

    // Test with the sha256= prefix (as per Toggl documentation)
    let signature_with_prefix = format!("sha256={}", hash);
    assert!(verify_signature(payload, &signature_with_prefix, secret));

    // Test backward compatibility (without prefix)
    assert!(verify_signature(payload, &hash, secret));
}

#[test]
fn test_event_serialization() {
    use chrono::Utc;
    use serde_json::json;
    use toggl_core::{UserId, WorkspaceId};

    use crate::events::{EventMetadata, WebhookEvent};

    let now = Utc::now();
    let event = WebhookEvent {
        event_id: "evt_123".to_string(),
        consumer_id: "sub_456".to_string(),
        last_delivery_attempt: now,
        last_delivery_error: None,
        failed_delivery_attempts: 0,
        subscription_id: 456,
        url_callback: "https://example.com/webhook".to_string(),
        timestamp: now,
        created_at: Some(now),
        creator_id: Some(UserId::new(789)),
        metadata: EventMetadata {
            path: "/api/v9/workspaces/123/time_entries".to_string(),
            model: "time_entry".to_string(),
            action: "created".to_string(),
            request_type: "POST".to_string(),
            workspace_id: WorkspaceId::new(123),
            event_user_id: UserId::new(789),
            entity_ids: json!({
                "time_entry_id": 999
            }),
            request_id: Some("req_789".to_string()),
            event_version: Some("1.0".to_string()),
        },
        payload: json!({
            "id": 999,
            "user_id": 789,
            "workspace_id": 123,
            "description": "Test time entry",
            "duration": 3600,
            "billable": false,
            "start": "2024-01-01T10:00:00Z",
            "at": "2024-01-01T10:00:00Z"
        }),
    };

    // Test that we can serialize and deserialize
    let json = serde_json::to_string(&event).unwrap();
    let deserialized: WebhookEvent = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.event_id, event.event_id);
    assert_eq!(deserialized.consumer_id, event.consumer_id);
    assert_eq!(deserialized.subscription_id, event.subscription_id);
    assert_eq!(deserialized.url_callback, event.url_callback);
    assert_eq!(deserialized.metadata.action, event.metadata.action);
    assert_eq!(deserialized.metadata.model, event.metadata.model);
    assert_eq!(deserialized.metadata.path, event.metadata.path);
    assert_eq!(
        deserialized.metadata.request_type,
        event.metadata.request_type
    );
}

#[test]
fn test_event_helper_functions() {
    use chrono::Utc;
    use serde_json::json;
    use toggl_core::{UserId, WorkspaceId};

    use crate::events::{
        EventAction, EventEntityType, EventMetadata, TimeEntryEventData, WebhookEvent,
    };

    let now = Utc::now();
    let event = WebhookEvent {
        event_id: "evt_123".to_string(),
        consumer_id: "sub_456".to_string(),
        last_delivery_attempt: now,
        last_delivery_error: None,
        failed_delivery_attempts: 0,
        subscription_id: 456,
        url_callback: "https://example.com/webhook".to_string(),
        timestamp: now,
        created_at: Some(now),
        creator_id: Some(UserId::new(789)),
        metadata: EventMetadata {
            path: "/api/v9/workspaces/123/time_entries".to_string(),
            model: "time_entry".to_string(),
            action: "created".to_string(),
            request_type: "POST".to_string(),
            workspace_id: WorkspaceId::new(123),
            event_user_id: UserId::new(789),
            entity_ids: json!({
                "time_entry_id": 999
            }),
            request_id: Some("req_789".to_string()),
            event_version: Some("1.0".to_string()),
        },
        payload: json!({
            "id": 999,
            "user_id": 789,
            "workspace_id": 123,
            "description": "Test time entry",
            "duration": 3600,
            "billable": false,
            "start": "2024-01-01T10:00:00Z",
            "stop": null,
            "at": "2024-01-01T10:00:00Z",
            "project_id": null,
            "task_id": null,
            "tags": null,
            "tag_ids": null
        }),
    };

    // Test event_type helper
    assert_eq!(event.event_type(), Some(EventEntityType::TimeEntry));

    // Test event_action helper
    assert_eq!(event.event_action(), Some(EventAction::Created));

    // Test is_event helper
    assert!(event.is_event(EventEntityType::TimeEntry, EventAction::Created));
    assert!(!event.is_event(EventEntityType::Project, EventAction::Created));
    assert!(!event.is_event(EventEntityType::TimeEntry, EventAction::Updated));

    // Test parse_payload helper
    let time_entry = event.parse_payload::<TimeEntryEventData>().unwrap();
    assert_eq!(time_entry.id.0, 999);
    assert_eq!(time_entry.user_id.0, 789);
    assert_eq!(time_entry.workspace_id.0, 123);
    assert_eq!(time_entry.description, Some("Test time entry".to_string()));
    assert_eq!(time_entry.duration, 3600);
    assert!(!time_entry.billable);
}

#[test]
fn test_event_type_matching() {
    use crate::events::EventEntityType;

    // Test all entity types
    let test_cases = vec![
        ("time_entry", Some(EventEntityType::TimeEntry)),
        ("project", Some(EventEntityType::Project)),
        ("client", Some(EventEntityType::Client)),
        ("tag", Some(EventEntityType::Tag)),
        ("task", Some(EventEntityType::Task)),
        ("user", Some(EventEntityType::User)),
        ("workspace_user", Some(EventEntityType::WorkspaceUser)),
        ("group", Some(EventEntityType::Group)),
        ("project_group", Some(EventEntityType::ProjectGroup)),
        ("project_user", Some(EventEntityType::ProjectUser)),
        ("unknown_type", None),
    ];

    for (model_name, expected) in test_cases {
        let event = create_test_event_with_model(model_name);
        assert_eq!(
            event.event_type(),
            expected,
            "Failed for model: {}",
            model_name
        );
    }
}

// Helper function for tests
fn create_test_event_with_model(model: &str) -> crate::events::WebhookEvent {
    use crate::events::{EventMetadata, WebhookEvent};
    use chrono::Utc;
    use serde_json::json;
    use toggl_core::{UserId, WorkspaceId};

    let now = Utc::now();
    WebhookEvent {
        event_id: "evt_test".to_string(),
        consumer_id: "sub_test".to_string(),
        last_delivery_attempt: now,
        last_delivery_error: None,
        failed_delivery_attempts: 0,
        subscription_id: 1,
        url_callback: "https://example.com".to_string(),
        timestamp: now,
        created_at: None,
        creator_id: None,
        metadata: EventMetadata {
            path: "/api/v9/test".to_string(),
            model: model.to_string(),
            action: "created".to_string(),
            request_type: "POST".to_string(),
            workspace_id: WorkspaceId::new(1),
            event_user_id: UserId::new(1),
            entity_ids: json!({}),
            request_id: None,
            event_version: None,
        },
        payload: json!({}),
    }
}
