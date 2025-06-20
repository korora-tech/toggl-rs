# toggl-core

Core functionality shared across all Toggl Rust client libraries.

## Overview

This crate provides the foundational components used by all Toggl client crates:
- Common types and traits
- Error handling
- Authentication
- Base HTTP client functionality

## Key Components

### BaseClient

The `BaseClient` struct provides common HTTP client functionality that is shared between `toggl-api` and `toggl-reports`:

```rust
use toggl_core::{BaseClient, Result};

// Create a base client
let client = BaseClient::new("your-api-token".to_string())?;

// Make requests
let response: MyType = client.get("https://api.example.com/endpoint", &params)?;
```

### Error Handling

Comprehensive error types for all API operations:
- `ApiError`: HTTP API errors with status codes
- `AuthError`: Authentication failures
- `RateLimitError`: Rate limiting with retry information
- `ValidationError`: Request validation errors
- `NetworkError`: Network and connection issues

### Authentication

Simple API token authentication:

```rust
use toggl_core::Auth;

let auth = Auth::api_token("your-api-token");
```

## Usage

This crate is primarily used as a dependency by other Toggl client crates:
- `toggl-api`: Main Toggl Track API client
- `toggl-reports`: Toggl Reports API client
- `toggl-webhooks`: Webhook models and utilities

## License

Same as the parent project.