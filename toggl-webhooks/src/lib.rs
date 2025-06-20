//! # Toggl Webhooks API Client
//!
//! Client library for the Toggl Track Webhooks API.
//!
//! This crate provides functionality for:
//! - Managing webhook subscriptions
//! - Configuring webhook endpoints
//! - Handling webhook events
//! - Event verification and security
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use toggl_webhooks::{WebhooksClient, models::*};
//! use toggl_core::{Result, WorkspaceId};
//!
//! # fn main() -> Result<()> {
//! // Create a client
//! let client = WebhooksClient::new("your-api-token".to_string())?;
//!
//! // Get subscriptions for a workspace
//! let workspace_id = WorkspaceId::new(12345);
//! let subscriptions = client.get_subscriptions(workspace_id)?;
//!
//! // Create a new subscription
//! let subscription = CreateSubscription {
//!     workspace_id,
//!     url_callback: "https://example.com/webhook".to_string(),
//!     enabled: Some(true),
//!     description: Some("My webhook".to_string()),
//!     event_filters: Some(vec![
//!         SubscriptionInEventFilter {
//!             entity: "time_entry".to_string(),
//!             action: "created".to_string(),
//!         },
//!         // You can also use wildcards
//!         SubscriptionInEventFilter {
//!             entity: "project".to_string(),
//!             action: "*".to_string(), // All actions
//!         },
//!         SubscriptionInEventFilter {
//!             entity: "*".to_string(), // All entities
//!             action: "deleted".to_string(),
//!         }
//!     ]),
//!     secret: Some("my-secret-key".to_string()),
//!     user_agent: None,
//!     has_pending_events: None,
//! };
//! let created = client.create_subscription(&subscription)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Webhook Event Verification
//!
//! ```rust,no_run
//! use toggl_webhooks::events::verify_signature;
//!
//! // In your webhook handler
//! fn handle_webhook(body: &[u8], signature: &str, secret: &str) -> bool {
//!     verify_signature(body, signature, secret)
//! }
//! ```

#[cfg(feature = "client")]
pub mod client;
pub mod events;
pub mod models;

#[cfg(all(test, feature = "client"))]
mod tests;

#[cfg(feature = "client")]
pub use client::{WebhooksClient, WEBHOOKS_API_BASE_URL};
#[cfg(feature = "client")]
pub use toggl_core::Auth;
pub use toggl_core::{Error, Result};

// Re-export webhook types
pub use models::{
    AppHealth, AppHealthOutDto, CreateSubscription, ErrorDto, EventFilters, EventFiltersGroupDto,
    EventFiltersOutDto, Subscription, SubscriptionInDto, SubscriptionInEventFilter,
    SubscriptionOutDto, SubscriptionOutEventFilter, SubscriptionUpdateDto,
    SubscriptionUpdateEventFilter, SubscriptionValidationDto, SubscriptionValidationResponseDto,
    UpdateSubscription, ValidationRequest, ValidationResponse,
};

#[cfg(feature = "client")]
pub use events::verify_signature;
pub use events::{
    ClientEventData, EventAction, EventEntityType, EventMetadata, EventPayload, GroupEventData,
    ProjectEventData, ProjectGroupEventData, ProjectUserEventData, TagEventData, TaskEventData,
    TimeEntryEventData, UserEventData, WebhookEvent, WorkspaceUserEventData,
};
