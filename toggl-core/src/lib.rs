//! # Toggl Core
//!
//! Core types and traits shared across all Toggl API client libraries.
//!
//! This crate provides the foundational types and utilities used across
//! all Toggl client crates.
//!
//! # Key Components
//!
//! - **Authentication**: The `Auth` struct for API authentication
//! - **Error Handling**: Comprehensive error types for all API operations
//! - **Common Types**: Shared types like IDs and timestamps
//!
//! # Error Handling
//!
//! The error system provides specific error types for different failure modes:
//! - `ApiError`: HTTP API errors with status codes
//! - `AuthError`: Authentication failures
//! - `RateLimitError`: Rate limiting with retry information
//! - `ValidationError`: Request validation errors
//! - `NetworkError`: Network and connection issues
//!
//! # Example
//!
//! ```rust,no_run
//! use toggl_core::{Auth, Error};
//!
//! // Create authentication
//! let auth = Auth::api_token("your-api-token");
//!
//! // Handle errors with specific types
//! fn handle_error(err: Error) {
//!     match err {
//!         Error::AuthError(msg) => eprintln!("Authentication failed: {}", msg),
//!         Error::RateLimitError { retry_after } => {
//!             eprintln!("Rate limited. Retry after {} seconds", retry_after);
//!         }
//!         _ => eprintln!("Other error: {}", err),
//!     }
//! }
//! ```

pub mod auth;
pub mod error;
pub mod types;

pub use auth::*;
pub use error::*;
pub use types::*;
