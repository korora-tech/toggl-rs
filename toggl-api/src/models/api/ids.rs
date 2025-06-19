//! Type-safe ID wrappers for Toggl API entities

use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! define_id {
    ($name:ident, $inner:ty, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub $inner);

        impl $name {
            /// Creates a new ID
            pub fn new(id: $inner) -> Self {
                Self(id)
            }

            /// Returns the inner value
            pub fn value(&self) -> $inner {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$inner> for $name {
            fn from(id: $inner) -> Self {
                Self(id)
            }
        }

        impl From<$name> for $inner {
            fn from(id: $name) -> $inner {
                id.0
            }
        }
    };
}

macro_rules! define_string_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            /// Creates a new ID
            pub fn new(id: String) -> Self {
                Self(id)
            }

            /// Returns the inner value
            pub fn value(&self) -> &str {
                &self.0
            }

            /// Consumes self and returns the inner String
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<String> for $name {
            fn from(id: String) -> Self {
                Self(id)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> String {
                id.0
            }
        }
    };
}

// Define numeric ID types (all using u64 for consistency)
define_id!(WorkspaceId, u64, "Workspace identifier");
define_id!(UserId, u64, "User identifier");
define_id!(ProjectId, u64, "Project identifier");
define_id!(ClientId, u64, "Client identifier");
define_id!(TimeEntryId, u64, "Time entry identifier");
define_id!(TaskId, u64, "Task identifier");
define_id!(TagId, u64, "Tag identifier");
define_id!(OrganizationId, u64, "Organization identifier");
define_id!(GroupId, u64, "Group identifier");
define_id!(AlertId, u64, "Alert identifier");
define_id!(GoalId, u64, "Goal identifier");
define_id!(SubscriptionId, u64, "Subscription identifier");
define_id!(PricingPlanId, u32, "Pricing plan identifier");
define_id!(CountryId, u32, "Country identifier");
define_id!(CurrencyId, u32, "Currency identifier");
define_id!(CreatorId, u64, "Creator (user) identifier");
define_id!(AssigneeId, u64, "Assignee (user) identifier");
define_id!(TimelineId, u64, "Timeline event identifier");
define_id!(DashboardId, u64, "Dashboard identifier");
define_id!(SmailId, u64, "Scheduled email identifier");
define_id!(AvatarId, u64, "Avatar identifier");

// Define string-based ID types
define_string_id!(InvitationId, "Invitation identifier");
define_string_id!(CalendarId, "Calendar identifier");
define_string_id!(ProviderUserId, "External provider user identifier");
define_string_id!(ApiKeyId, "API key identifier");
define_string_id!(ExportId, "Export identifier");
define_string_id!(InvoiceId, "Invoice identifier");
define_string_id!(SamlConfigurationId, "SAML configuration identifier");
