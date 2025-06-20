# toggl-rs Project Guide

This document provides essential information for working with the toggl-rs project - a Rust client library for the Toggl Track API v9.

## Project Overview

toggl-rs is a comprehensive Rust client library that provides type-safe access to the Toggl Track API v9. It supports authentication, time tracking, project management, reporting, and various other Toggl features.

### Architecture

The project uses a workspace structure with two main crates:
- **toggl-core**: Core functionality, error types, and common utilities
- **toggl-api**: API client implementation with all endpoints and models

### Key Design Decisions

1. **Model Organization**: Models are separated from client logic in `src/model/api/`
2. **Error Handling**: Comprehensive error types with specific variants for different failure scenarios
3. **Testing**: All endpoints have corresponding tests using mockito for HTTP mocking
4. **Type Safety**: Strong typing throughout with proper use of ID types (WorkspaceId, ProjectId, etc.)

## Development Setup

### Prerequisites
- Rust 1.70+ 
- Cargo
- Just (command runner)
- cargo-nextest (required for running tests)
  ```bash
  cargo install cargo-nextest
  ```

### Quick Start
```bash
# Check that everything is set up correctly (fmt, clippy, tests)
just check

# Run all tests
cargo nextest run

# Run tests with output
cargo nextest run --no-capture

# Build the project
cargo build

# Build release version
cargo build --release
```

### Available Features

The library provides feature flags to control what functionality is included:

```toml
[dependencies]
toggl-api = { version = "0.1", features = ["full"] }
```

Available features:
- `default`: Core API functionality
- `reports`: Reporting API support
- `webhooks`: Webhook support (placeholder)
- `minimal`: Just core API without extras
- `analytics`: API + reports
- `full`: All features enabled

### Using the Library

```rust
use toggl_api::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create client with API token
    let client = TogglClient::new("your-api-token");
    
    // Get current user
    let me = client.get_me().await?;
    println!("Hello, {}!", me.fullname);
    
    // List workspaces
    let workspaces = client.get_workspaces().await?;
    
    // Create a time entry
    let entry = client.create_time_entry(
        workspaces[0].id,
        CreateTimeEntry {
            description: Some("Working on toggl-rs".to_string()),
            start: Utc::now(),
            ..Default::default()
        }
    ).await?;
    
    Ok(())
}
```

## Project Structure

```
toggl-rs/
├── toggl-core/          # Core functionality crate
│   ├── src/
│   │   ├── error.rs     # Comprehensive error types
│   │   ├── lib.rs       # Core exports
│   │   └── types.rs     # Common type definitions
│   └── Cargo.toml
├── toggl-api/           # API client crate
│   ├── src/
│   │   ├── client/      # API client modules (endpoint implementations only)
│   │   │   ├── mod.rs   # Main client implementation
│   │   │   ├── me.rs    # User endpoints
│   │   │   ├── workspaces.rs # Workspace endpoints
│   │   │   ├── projects.rs  # Project endpoints
│   │   │   ├── time_entries.rs # Time entry endpoints
│   │   │   └── ...      # Other endpoint modules
│   │   ├── models.rs    # Model re-exports and organization
│   │   ├── prelude.rs   # Common imports for users
│   │   └── lib.rs       # Library entry point
│   └── Cargo.toml
├── examples/            # Usage examples
├── spec/               # API specifications
│   └── toggl-api-spec.json # OpenAPI specification
├── Cargo.toml          # Workspace configuration
└── Justfile            # Just commands
```

### Model Files Location

All API models are in `toggl-api/src/models/`:
- `auth.rs` - Authentication models
- `me.rs` - User/Me endpoint models
- `workspaces.rs` - Workspace-related models
- `projects.rs` - Project models
- `time_entries.rs` - Time entry models
- `organizations.rs` - Organization models
- `reports.rs` - Report models
- `dashboard.rs` - Dashboard models
- And many more...

## Key Commands

### Code Quality Checks
```bash
# Run all checks (fmt, clippy, nextest)
just check

# Format code
cargo fmt

# Run clippy linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo nextest run

# Run specific test
cargo nextest run test_name

# Run tests for a specific module
cargo nextest run tests::workspaces::
```

### Development Workflow
```bash
# Run example
cargo run --example basic_usage

# Build documentation
cargo doc --open

# Check for outdated dependencies
cargo outdated

# Check API endpoints in spec
jq '.paths | keys[]' spec/toggl-api-spec.json | sort | uniq
```

## API Implementation Status

### Fully Implemented Modules
- ✅ Authentication (auth)
- ✅ User/Me endpoints
- ✅ Workspaces (including clients, tags, alerts)
- ✅ Projects (including tasks)
- ✅ Time Entries (including bulk operations)
- ✅ Organizations (all endpoints)
- ✅ Reports
- ✅ Countries & Currencies
- ✅ Timezones
- ✅ Status
- ✅ Avatars
- ✅ Feedback
- ✅ Desktop Login
- ✅ API Keys
- ✅ Subscriptions
- ✅ Timeline
- ✅ Sync Server (goals)
- ✅ Timesheets
- ✅ Invoices (organization and workspace)
- ✅ SSO/SAML
- ✅ Calendar integrations
- ✅ Exports
- ✅ Invitations
- ✅ Favorites
- ✅ Audit logs

### Not Yet Implemented
- ❌ Webhooks (API spec does not include webhook endpoints)
- ❌ Some advanced reporting features (if any remain in spec)

## Testing Guidelines

### Running Tests
All API endpoints have corresponding tests in `src/tests/`. Tests use mockito for HTTP mocking.

```bash
# Run all tests
cargo nextest run

# Run with verbose output
cargo nextest run --no-capture

# Run specific test module
cargo nextest run tests::time_entries::

# Run single test
cargo nextest run test_create_time_entry

# Run failed tests only
cargo nextest run --failed

# Run tests in parallel (default) or limit jobs
cargo nextest run -j 4
```

### Writing Tests
Tests follow this pattern:
```rust
#[test]
fn test_endpoint_name() -> Result<()> {
    let response = json!({ /* mock response */ });
    
    with_mockito(
        Method::Get,
        "/endpoint/path",
        200,
        Some(response),
        |client| {
            // Test implementation
            Ok(())
        },
    )
}
```

## Common Tasks

### Adding a New Endpoint
1. Add the method to the appropriate client module in `toggl-api/src/client/`
2. Add necessary types to `toggl-api/src/models/` (create a new file if needed)
3. Update `toggl-api/src/models.rs` to declare and re-export the new types
4. Write tests in the corresponding test module
5. Run `just check` to ensure everything passes
6. Update this documentation if needed

### Code Organization
- **Client modules** (`toggl-api/src/client/`): Only contain client structs and API endpoint implementations
- **Model modules** (`toggl-api/src/models/`): Contain all request/response structs with `#[derive(Serialize, Deserialize)]`
- **Core types** (`toggl-core/src/`): Error types and common utilities
- Keep separation of concerns: models are separate from client logic

### Model Namespacing
Due to conflicting type names in the API, some models are organized into namespaces:
- `organization::` - Organization-specific types (e.g., `organization::Group`)
- `workspace_types::` - Workspace-specific types that conflict with organization
- `goals_types::` - Goal types from different modules
- `subscription_types::` - Subscription-related types

### Checking API Coverage
Compare implemented endpoints against `spec/toggl-api-spec.json`:
```bash
# List all endpoints in spec
jq '.paths | keys[]' spec/toggl-api-spec.json | sort

# Check which endpoints are implemented
grep -r "pub fn" src/client/ | grep -v "new("
```

### Debugging API Requests
The client prints requests in test mode:
```rust
#[cfg(test)]
println!("Sending request to: {} {}", method, url);
```

## Code Style

### Formatting
- Use `cargo fmt` before committing
- Follow Rust naming conventions
- Add doc comments for public APIs

### Error Handling
- Use the custom `Result<T>` type from `error.rs`
- Return meaningful error messages
- Handle API errors appropriately

### Testing
- Write tests for all new endpoints
- Use realistic mock data
- Test error cases
- Ensure consistent parameter ordering for URL query strings

## Troubleshooting

### Common Issues

1. **Test failures due to parameter ordering**
   - HashMap doesn't guarantee order for query parameters
   - Use mockito's query matchers for tests with query params
   - Run `cargo nextest run --no-capture` to see actual vs expected URLs
   - Example fix:
   ```rust
   // Instead of hardcoding query string order
   "/api/v9/workspaces/123/projects?active=true&since=123"
   
   // Use mockito matchers
   server.mock("GET", Matcher::AllOf(vec![
       Matcher::Path("/api/v9/workspaces/123/projects"),
       Matcher::Query("active=true".to_string()),
       Matcher::Query("since=123".to_string()),
   ]))
   ```

2. **Missing fields in responses**
   - Check if fields are optional in the model (`Option<T>`)
   - Verify against `spec/toggl-api-spec.json`
   - Some fields may be null in certain contexts

3. **Authentication errors**
   - Ensure API token is properly formatted
   - Use "api_token" as password in BasicAuth
   - Token should be base64 encoded when sent

4. **Serialization errors**
   - Check for `#[serde(rename = "field_name")]` attributes
   - Verify DateTime formats match API expectations
   - Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields

### Debugging Tips
- Run tests with `cargo nextest run --no-capture` to see printed output
- Use `cargo nextest run --failed` to re-run only failed tests
- Check the `spec/toggl-api-spec.json` for endpoint details
- Use `cargo expand` to debug macro expansions
- Add `dbg!()` or `println!()` in tests to inspect values
- Use `RUST_LOG=debug` for more verbose output

## Contributing

1. Check existing issues and PRs
2. Run `just check` before submitting
3. Add tests for new functionality
4. Update documentation as needed
5. Follow existing code patterns

## Resources

- [Toggl API Documentation](https://developers.track.toggl.com/docs/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)

## Notes for AI Assistants

When working on this codebase:
1. Always run `just check` to ensure code quality (fmt, clippy, tests)
2. Check `spec/toggl-api-spec.json` for API endpoint details and model definitions
3. Follow existing patterns in client modules - look at similar endpoints first
4. Write comprehensive tests for new endpoints - test both success and error cases
5. Update model types as needed in `toggl-api/src/models/`
6. Consider parameter ordering in tests (HashMap is unordered) - use mockito query matchers
7. Use the existing error handling pattern from `toggl-core`
8. Add doc comments for all public APIs with examples when appropriate
9. For tests with query parameters, use mockito's query matchers to handle unordered parameters
10. When implementing new endpoints, ensure all tests pass before marking as complete
11. **Important**: Model structs belong in `toggl-api/src/models/`, not in client files
12. Client modules should only import models, not define them
13. Use the prelude for common imports: `use toggl_api::prelude::*;`
14. Check for existing similar endpoints before implementing - reuse patterns
15. Run `cargo nextest run --no-capture` to debug test failures
16. Use strong typing - prefer specific ID types over generic i64
17. Handle optional fields properly - use `Option<T>` for nullable API fields

### Quick Reference for Common Patterns

**Client method pattern:**
```rust
pub async fn get_something(&self, workspace_id: WorkspaceId) -> Result<Something> {
    let url = format!("{}/workspaces/{}/something", self.base_url, workspace_id);
    self.client.get(&url).send().await?.json().await
}
```

**Test pattern:**
```rust
#[test]
fn test_get_something() -> Result<()> {
    let response = json!({"id": 123, "name": "Test"});
    
    with_mockito(
        Method::Get,
        "/workspaces/123/something",
        200,
        Some(response),
        |client| {
            let result = client.get_something(WorkspaceId(123))?;
            assert_eq!(result.id, 123);
            Ok(())
        },
    )
}
```