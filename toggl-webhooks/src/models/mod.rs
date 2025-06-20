//! Webhook models and types
//!
//! This module contains all the request and response types for the Toggl Webhooks API.

mod error;
mod event_filters;
mod health;
mod subscriptions;
mod validation;

// Re-export types with clear names to avoid conflicts
pub use error::ErrorDto;
pub use event_filters::{EventFiltersGroupDto, EventFiltersOutDto};
pub use health::AppHealthOutDto;
pub use subscriptions::{
    SubscriptionInDto, SubscriptionInEventFilter, SubscriptionOutDto, SubscriptionOutEventFilter,
    SubscriptionUpdateDto, SubscriptionUpdateEventFilter,
};
pub use validation::{SubscriptionValidationDto, SubscriptionValidationResponseDto};

// Type aliases for convenience and backwards compatibility
pub type CreateSubscription = SubscriptionInDto;
pub type UpdateSubscription = SubscriptionUpdateDto;
pub type Subscription = SubscriptionOutDto;
pub type EventFilters = EventFiltersOutDto;
pub type AppHealth = AppHealthOutDto;
pub type ValidationRequest = SubscriptionValidationDto;
pub type ValidationResponse = SubscriptionValidationResponseDto;
