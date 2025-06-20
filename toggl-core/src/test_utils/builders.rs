use serde_json::{json, Value};

pub struct TestWorkspaceBuilder {
    id: u64,
    name: String,
    premium: bool,
    organization_id: Option<u64>,
}

impl Default for TestWorkspaceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TestWorkspaceBuilder {
    pub fn new() -> Self {
        Self {
            id: super::TEST_WORKSPACE_ID,
            name: "Test Workspace".to_string(),
            premium: false,
            organization_id: None,
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn premium(mut self, premium: bool) -> Self {
        self.premium = premium;
        self
    }

    pub fn organization_id(mut self, org_id: u64) -> Self {
        self.organization_id = Some(org_id);
        self
    }

    pub fn build(self) -> Value {
        let mut workspace = json!({
            "id": self.id,
            "name": self.name,
            "premium": self.premium,
            "profile": 0,
            "business_ws": false,
            "admin": true,
            "default_hourly_rate": null,
            "rate_last_updated": null,
            "default_currency": "USD",
            "only_admins_may_create_projects": false,
            "only_admins_may_create_tags": false,
            "only_admins_see_billable_rates": false,
            "only_admins_see_team_dashboard": false,
            "projects_billable_by_default": true,
            "projects_enforce_billable": false,
            "projects_private_by_default": false,
            "reports_collapse": true,
            "rounding": 1,
            "rounding_minutes": 0,
            "api_token": null,
            "at": "2022-10-03T15:44:00.289146Z",
            "ical_enabled": false,
            "ical_url": null,
            "csv_upload": null,
            "subscription": null,
            "working_hours_in_minutes": null,
            "logo_url": null,
            "permissions": null,
            "max_data_retention_days": null
        });

        if let Some(org_id) = self.organization_id {
            workspace["organization_id"] = json!(org_id);
        }

        workspace
    }
}

pub struct TestProjectBuilder {
    id: u64,
    workspace_id: u64,
    name: String,
    client_id: Option<u64>,
    active: bool,
    color: String,
}

impl Default for TestProjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TestProjectBuilder {
    pub fn new() -> Self {
        Self {
            id: super::TEST_PROJECT_ID,
            workspace_id: super::TEST_WORKSPACE_ID,
            name: "Test Project".to_string(),
            client_id: None,
            active: true,
            color: "#06aaf5".to_string(),
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn workspace_id(mut self, workspace_id: u64) -> Self {
        self.workspace_id = workspace_id;
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn client_id(mut self, client_id: u64) -> Self {
        self.client_id = Some(client_id);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    pub fn build(self) -> Value {
        let mut project = json!({
            "id": self.id,
            "workspace_id": self.workspace_id,
            "name": self.name,
            "active": self.active,
            "private": true,
            "template": false,
            "billable": true,
            "is_private": true,
            "auto_estimates": false,
            "estimated_hours": null,
            "color": self.color,
            "rate": null,
            "created_at": "2024-01-15T10:00:00+00:00",
            "updated_at": "2024-01-15T10:00:00+00:00",
            "recurring": false,
            "current_period": null,
            "fixed_fee": null,
            "actual_hours": 0
        });

        if let Some(client_id) = self.client_id {
            project["client_id"] = json!(client_id);
        }

        project
    }
}

pub struct TestUserBuilder {
    id: u64,
    email: String,
    fullname: String,
    default_workspace_id: u64,
}

impl Default for TestUserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TestUserBuilder {
    pub fn new() -> Self {
        Self {
            id: super::TEST_USER_ID,
            email: "test@example.com".to_string(),
            fullname: "Test User".to_string(),
            default_workspace_id: super::TEST_WORKSPACE_ID,
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub fn fullname(mut self, fullname: impl Into<String>) -> Self {
        self.fullname = fullname.into();
        self
    }

    pub fn default_workspace_id(mut self, workspace_id: u64) -> Self {
        self.default_workspace_id = workspace_id;
        self
    }

    pub fn build(self) -> Value {
        json!({
            "id": self.id,
            "api_token": "1234567890abcdef",
            "email": self.email,
            "fullname": self.fullname,
            "timezone": "Europe/London",
            "toggl_accounts_id": "1234567890abcdef",
            "default_workspace_id": self.default_workspace_id,
            "beginning_of_week": 1,
            "image_url": "https://assets.track.toggl.com/images/profile.png",
            "created_at": "2024-01-15T10:00:00+00:00",
            "updated_at": "2024-01-15T10:00:00+00:00",
            "at": "2024-01-15T10:00:00+00:00"
        })
    }
}

pub struct TestTimeEntryBuilder {
    id: u64,
    workspace_id: u64,
    user_id: u64,
    project_id: Option<u64>,
    description: String,
    duration: u64,
}

impl Default for TestTimeEntryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TestTimeEntryBuilder {
    pub fn new() -> Self {
        Self {
            id: super::TEST_TIME_ENTRY_ID,
            workspace_id: super::TEST_WORKSPACE_ID,
            user_id: super::TEST_USER_ID,
            project_id: None,
            description: "Test time entry".to_string(),
            duration: 3600, // 1 hour
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn workspace_id(mut self, workspace_id: u64) -> Self {
        self.workspace_id = workspace_id;
        self
    }

    pub fn user_id(mut self, user_id: u64) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn project_id(mut self, project_id: u64) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn duration(mut self, duration: u64) -> Self {
        self.duration = duration;
        self
    }

    pub fn build(self) -> Value {
        let mut entry = json!({
            "id": self.id,
            "workspace_id": self.workspace_id,
            "user_id": self.user_id,
            "description": self.description,
            "duration": self.duration,
            "start": "2024-01-15T10:00:00+00:00",
            "stop": "2024-01-15T11:00:00+00:00",
            "duronly": false,
            "billable": false,
            "created_with": "toggl-rs",
            "tags": [],
            "at": "2024-01-15T10:00:00+00:00"
        });

        if let Some(project_id) = self.project_id {
            entry["project_id"] = json!(project_id);
        }

        entry
    }
}

// Helper functions for common test responses
pub fn empty_list_response() -> Value {
    json!([])
}

pub fn error_response(status: u16, message: &str) -> Value {
    json!({
        "error": message,
        "status": status
    })
}

pub fn success_response<T: serde::Serialize>(data: T) -> Value {
    serde_json::to_value(data).unwrap()
}
