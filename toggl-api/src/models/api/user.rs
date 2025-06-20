use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use toggl_core::{CountryId, TagId, UserId, WorkspaceId};

/// Represents a Toggl user account.
///
/// This struct contains all the information associated with a user profile,
/// including authentication details, personal settings, and workspace preferences.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let user = client.get_me().await?;
/// println!("Hello, {}!", user.fullname);
/// println!("Email: {}", user.email);
/// println!("Default workspace: {}", user.default_workspace_id);
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// Unique identifier for the user
    pub id: UserId,
    /// User's API token for authentication (only visible to the user themselves)
    pub api_token: Option<String>,
    /// User's email address
    pub email: String,
    /// User's full name
    pub fullname: String,
    /// User's timezone (e.g., "America/New_York", "Europe/London")
    pub timezone: String,
    /// ID of the user's default workspace
    pub default_workspace_id: WorkspaceId,
    /// Day of week the user's week starts on (0 = Sunday, 1 = Monday, etc.)
    pub beginning_of_week: u8,
    /// URL to the user's profile image
    pub image_url: String,
    /// Timestamp when the user account was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the user account was last updated
    pub updated_at: DateTime<Utc>,
    /// OpenID email if SSO is enabled
    pub openid_email: Option<String>,
    /// Whether OpenID/SSO is enabled for this user
    pub openid_enabled: bool,
    /// User's country ID for localization
    pub country_id: Option<CountryId>,
    /// Server timestamp when this data was retrieved
    pub at: DateTime<Utc>,
    /// Hash for Intercom integration
    pub intercom_hash: Option<String>,
    /// Whether the user has set a password (false if using SSO only)
    pub has_password: bool,
    /// Additional user options and preferences
    pub options: Option<Value>,
    /// User's personal tags (deprecated, use workspace tags instead)
    pub tags: Option<Vec<Tag>>,
}

/// Represents a personal tag associated with a user.
///
/// **Note**: Personal tags are deprecated. Use workspace tags instead for better
/// collaboration and organization-wide consistency.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    /// Unique identifier for the tag
    pub id: TagId,
    /// Name of the tag
    pub name: String,
    /// Workspace this tag belongs to
    pub workspace_id: WorkspaceId,
}

/// Request structure for updating user profile information.
///
/// All fields are optional - only include the fields you want to update.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
/// use toggl_api::models::api::user::UpdateUser;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let update = UpdateUser {
///     fullname: Some("John Doe".to_string()),
///     timezone: Some("Europe/London".to_string()),
///     beginning_of_week: Some(1), // Monday
///     ..Default::default()
/// };
/// let updated_user = client.update_me(update).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateUser {
    /// New full name
    pub fullname: Option<String>,
    /// New email address
    pub email: Option<String>,
    /// New timezone (IANA timezone identifier)
    pub timezone: Option<String>,
    /// New default workspace ID
    pub default_workspace_id: Option<WorkspaceId>,
    /// New beginning of week (0-6, where 0 = Sunday)
    pub beginning_of_week: Option<u8>,
    /// New country ID
    pub country_id: Option<CountryId>,
    /// New password (requires current_password)
    pub password: Option<String>,
    /// Current password (required when changing password)
    pub current_password: Option<String>,
}

/// Represents web timer information for a user.
///
/// Web timers track time spent on web-based activities through browser extensions
/// or integrations.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserWebTimer {
    /// Unique identifier for the web timer
    pub web_timer_id: String,
    /// Total seconds tracked by this web timer
    pub web_timer_seconds: i64,
}

/// Request structure for initiating password reset.
///
/// Used to request a password reset email for the given email address.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LostPassword {
    /// Email address associated with the account
    pub email: String,
}

/// Request structure for completing password reset.
///
/// Used with the token received via email to set a new password.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetToken {
    /// Password reset token from email
    pub token: String,
    /// New password to set
    pub password: String,
    /// Email address associated with the account
    pub email: String,
}

/// Request structure for creating a new Toggl account.
///
/// Contains all necessary information for user registration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupData {
    /// Email address for the new account
    pub email: String,
    /// Password for the new account
    pub password: String,
    /// User's timezone (IANA timezone identifier)
    pub timezone: Option<String>,
    /// Source of account creation (e.g., "web", "mobile")
    pub created_with: Option<String>,
    /// Whether user agreed to terms of service
    pub terms_agreed: Option<bool>,
    /// Timestamp when terms were agreed to
    pub terms_agreed_at: Option<DateTime<Utc>>,
    /// Whether user agreed to receive marketing emails
    pub marketing_agreed: Option<bool>,
    /// Timestamp when marketing agreement was made
    pub marketing_agreed_at: Option<DateTime<Utc>>,
    /// Alternative field for terms of service acceptance
    pub tos_accepted: Option<bool>,
    /// User's country for localization
    pub country_id: Option<CountryId>,
}
