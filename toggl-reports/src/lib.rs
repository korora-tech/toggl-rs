//! # Toggl Reports API Client
//!
//! Client library for the Toggl Track Reports API.
//!
//! This crate provides access to Toggl's reporting functionality including:
//! - Summary reports
//! - Detailed reports
//! - Weekly reports
//! - Time entry reports
//! - Project and task analytics

pub mod client;
pub mod models;

pub use client::ReportsClient;
pub use toggl_core::{Auth, Error, Result};

// Re-export report models
