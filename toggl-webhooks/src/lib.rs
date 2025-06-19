//! # Toggl Webhooks API Client
//!
//! Client library for the Toggl Track Webhooks API.
//!
//! This crate provides functionality for:
//! - Managing webhook subscriptions
//! - Configuring webhook endpoints
//! - Handling webhook events
//! - Event verification and security

pub mod client;
pub mod events;
pub mod models;

pub use client::WebhooksClient;
pub use toggl_core::{Auth, Error, Result};

// Re-export webhook types
