# toggl-rs

A comprehensive Rust client library for the [Toggl Track API v9](https://track.toggl.com) with 100% API coverage.

## Features

- **Complete API Coverage**: All 275 Toggl Track API v9 endpoints are fully implemented
- **Type-safe**: Strongly typed request and response models
- **Synchronous**: Uses `reqwest::blocking` for straightforward usage
- **Well-tested**: Comprehensive test suite with mockito
- **Multipart Support**: Full support for file uploads including logos and expense receipts

## API Documentation

For detailed API endpoint documentation, please refer to:
- [Toggl API Documentation](https://developers.track.toggl.com/docs/)
- [Toggl Engineering Docs](https://engineering.toggl.com/docs/)

## Usage

```rust
use toggl_rs::{Client, BasicAuth};

// Create a client with your API token
let auth = BasicAuth::new("your-api-token");
let client = Client::new(auth);

// Get current user
let me = client.me().get()?;
println!("Hello, {}!", me.fullname);

// List workspaces
let workspaces = client.me().get_workspaces()?;
for workspace in workspaces {
    println!("Workspace: {}", workspace.name);
}
```

## Modules

The client is organized into modules that correspond to the Toggl API sections:

- `me` - User profile and preferences
- `workspaces` - Workspace management
- `projects` - Project operations
- `time_entries` - Time tracking
- `organizations` - Organization management
- `reports` - Reporting endpoints
- `auth` - Authentication (SAML)
- And many more...

## Examples

See the `examples/` directory for more usage examples.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
