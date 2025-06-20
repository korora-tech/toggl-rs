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
    use toggl_core::UserId;

    use crate::events::{EventMetadata, WebhookEvent};

    let event = WebhookEvent {
        event_id: "evt_123".to_string(),
        created_at: Utc::now(),
        creator_id: Some(UserId::new(456)),
        metadata: EventMetadata {
            action: "created".to_string(),
            entity_type: "time_entry".to_string(),
            request_id: Some("req_789".to_string()),
            event_version: Some("1.0".to_string()),
        },
        payload: json!({
            "id": 123,
            "user_id": 456,
            "workspace_id": 789,
            "description": "Test time entry",
            "duration": 3600,
            "billable": false
        }),
    };

    // Test that we can serialize and deserialize
    let json = serde_json::to_string(&event).unwrap();
    let deserialized: WebhookEvent = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.event_id, event.event_id);
    assert_eq!(deserialized.metadata.action, event.metadata.action);
    assert_eq!(
        deserialized.metadata.entity_type,
        event.metadata.entity_type
    );
}
