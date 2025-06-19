# toggl-rs Project Guide

This document provides essential information for working with the toggl-rs project - a Rust client library for the Toggl Track API v9.

## Project Overview

toggl-rs is a comprehensive Rust client library that provides type-safe access to the Toggl Track API v9. It supports authentication, time tracking, project management, reporting, and various other Toggl features.

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

## Project Structure

```
toggl-rs/
├── src/
│   ├── client/           # API client modules (endpoint implementations only)
│   │   ├── mod.rs       # Main client implementation
│   │   ├── me.rs        # User endpoints
│   │   ├── workspaces.rs # Workspace endpoints
│   │   ├── projects.rs  # Project endpoints
│   │   ├── time_entries.rs # Time entry endpoints
│   │   └── ...          # Other endpoint modules
│   ├── model/           # Data models
│   │   ├── api/         # API request/response models
│   │   │   ├── mod.rs   # Module declarations and re-exports
│   │   │   ├── audit.rs # Audit log models
│   │   │   ├── auth.rs  # Authentication models
│   │   │   └── ...      # Other model files
│   │   ├── reports/     # Report-specific models
│   │   └── webhooks/    # Webhook models (placeholder)
│   ├── tests/           # Unit tests
│   ├── error.rs         # Error handling
│   └── lib.rs          # Library entry point
├── examples/            # Usage examples
├── spec/               # API specifications
│   └── toggl-api-spec.json # OpenAPI specification
├── Cargo.toml          # Rust dependencies
└── Justfile            # Just commands
```

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
1. Add the method to the appropriate client module in `src/client/`
2. Add necessary types to `src/model/api/` (create a new file if needed)
3. Update `src/model/api/mod.rs` to declare the module and re-export types
4. Write tests in `src/tests/`
5. Update this documentation if needed

### Code Organization
- **Client modules** (`src/client/`): Only contain client structs and API endpoint implementations
- **Model modules** (`src/model/api/`): Contain all request/response structs with `#[derive(Serialize, Deserialize)]`
- Keep separation of concerns: models are separate from client logic

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
   - Match the actual order in tests or use path-only matching
   - Run `cargo nextest run --no-capture` to see actual vs expected URLs

2. **Missing fields in responses**
   - Check if fields are optional in the model
   - Verify against API documentation

3. **Authentication errors**
   - Ensure API token is properly formatted
   - Use "api_token" as password in BasicAuth

### Debugging Tips
- Run tests with `cargo nextest run --no-capture` to see printed output
- Use `cargo nextest run --failed` to re-run only failed tests
- Check the `spec/toggl-api-spec.json` for endpoint details
- Use `cargo expand` to debug macro expansions

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
1. Always run `just check` to ensure code quality
2. Check `spec/toggl-api-spec.json` for API endpoint details
3. Follow existing patterns in client modules
4. Write comprehensive tests for new endpoints
5. Update model types as needed
6. Consider parameter ordering in tests (HashMap is unordered)
7. Use the existing error handling pattern
8. Add doc comments for public APIs
9. For tests with query parameters, use mockito's query matchers to handle unordered parameters
10. When implementing new endpoints, check all tests pass before marking as complete
11. **Important**: Model structs (request/response types) belong in `src/model/api/`, not in client files
12. Client modules should only import models, not define them