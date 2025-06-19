#[cfg(test)]
mod tests {
    use crate::models::api::calendar::*;
    use crate::tests::*;
    use chrono::DateTime;
    use reqwest::Method;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn test_update_integration() -> Result<()> {
        let update = UpdateIntegration {
            enabled: Some(true),
        };

        let response = json!({
            "id": 123,
            "provider": "google",
            "provider_user_id": "user@gmail.com",
            "linked_at": "2024-01-01T10:00:00Z",
            "updated_at": "2024-01-16T10:00:00Z"
        });

        with_mockito(
            Method::PUT,
            "/integrations/calendar/123",
            200,
            Some(response),
            |client| {
                let integration = client.calendar().update_integration(123, &update)?;
                assert_eq!(integration.id, 123);
                assert_eq!(integration.provider, "google");
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_integration_calendars() -> Result<()> {
        let response = json!({
            "calendars": [
                {
                    "id": "primary",
                    "integration_id": 123,
                    "name": "My Calendar",
                    "description": "Personal calendar",
                    "selected": true,
                    "color": "#4285F4"
                },
                {
                    "id": "work",
                    "integration_id": 123,
                    "name": "Work Calendar",
                    "selected": false,
                    "color": "#0F9D58"
                }
            ],
            "next_page_token": null
        });

        with_mockito(
            Method::GET,
            "/integrations/calendar/123/calendars",
            200,
            Some(response),
            |client| {
                let result = client.calendar().get_integration_calendars(123)?;
                assert_eq!(result.calendars.len(), 2);
                assert_eq!(result.calendars[0].name, "My Calendar");
                assert!(result.calendars[0].selected);
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_integration_calendars() -> Result<()> {
        let update = CalendarUpdateRequest {
            calendars: vec![
                CalendarUpdate {
                    id: "primary".to_string(),
                    selected: true,
                },
                CalendarUpdate {
                    id: "work".to_string(),
                    selected: false,
                },
            ],
        };

        with_mockito(
            Method::POST,
            "/integrations/calendar/123/calendars/update",
            200,
            None,
            |client| {
                client
                    .calendar()
                    .update_integration_calendars(123, &update)?;
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_calendar() -> Result<()> {
        let update = CalendarUpdate {
            id: "primary".to_string(),
            selected: true,
        };

        let response = json!({
            "id": "primary",
            "integration_id": 123,
            "name": "My Calendar",
            "selected": true,
            "color": "#4285F4"
        });

        with_mockito(
            Method::PATCH,
            "/integrations/calendar/123/calendars/primary",
            200,
            Some(response),
            |client| {
                let calendar = client.calendar().update_calendar(123, "primary", &update)?;
                assert_eq!(calendar.id, "primary");
                assert!(calendar.selected);
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_calendar_events() -> Result<()> {
        let response = json!({
            "events": [
                {
                    "id": "evt1",
                    "calendar_id": "primary",
                    "integration_id": 123,
                    "title": "Team Meeting",
                    "description": "Weekly sync",
                    "start": "2024-01-16T14:00:00Z",
                    "end": "2024-01-16T15:00:00Z",
                    "all_day": false,
                    "recurring": true,
                    "location": "Conference Room A"
                }
            ],
            "next_page_token": null
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-16");
        params.insert("end_date", "2024-01-17");

        with_mockito_params(
            Method::GET,
            "/integrations/calendar/123/calendars/primary/events",
            Some(params),
            200,
            Some(response),
            |client| {
                let result = client.calendar().get_calendar_events(
                    123,
                    "primary",
                    "2024-01-16",
                    "2024-01-17",
                )?;

                assert_eq!(result.events.len(), 1);
                assert_eq!(result.events[0].title, "Team Meeting");
                assert!(result.events[0].recurring);
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_event_details_suggestion() -> Result<()> {
        let response = json!({
            "project_id": 456,
            "task_id": 789,
            "billable": true,
            "description": "Suggested description",
            "workspace_id": 123
        });

        with_mockito(
            Method::GET,
            "/integrations/calendar/events/evt1/details-suggestion",
            200,
            Some(response),
            |client| {
                let suggestion = client.calendar().get_event_details_suggestion("evt1")?;
                assert_eq!(suggestion.project_id, Some(456));
                assert_eq!(suggestion.billable, Some(true));
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_events_details_suggestions() -> Result<()> {
        let response = json!([
            {
                "project_id": 456,
                "task_id": 789,
                "billable": true,
                "description": "Meeting notes",
                "workspace_id": 123
            },
            {
                "project_id": 457,
                "task_id": null,
                "billable": false,
                "description": "Internal sync",
                "workspace_id": 123
            }
        ]);

        with_mockito(
            Method::POST,
            "/integrations/calendar/events/details-suggestion",
            200,
            Some(response),
            |client| {
                let event_ids = vec!["evt1".to_string(), "evt2".to_string()];
                let suggestions = client
                    .calendar()
                    .get_events_details_suggestions(&event_ids)?;
                assert_eq!(suggestions.len(), 2);
                assert_eq!(suggestions[0].project_id, Some(456));
                assert_eq!(suggestions[1].billable, Some(false));
                Ok(())
            },
        )
    }

    #[test]
    fn test_calendar_callback() -> Result<()> {
        let response = json!({
            "id": 124,
            "provider": "google",
            "provider_user_id": "newuser@gmail.com",
            "linked_at": "2024-01-16T10:00:00Z",
            "updated_at": "2024-01-16T10:00:00Z"
        });

        let mut params = BTreeMap::new();
        params.insert("code", "auth_code_123");
        params.insert("state", "state_123");

        with_mockito_params(
            Method::GET,
            "/integrations/calendar/callback/google",
            Some(params),
            200,
            Some(response),
            |client| {
                let integration = client.calendar().calendar_callback(
                    "google",
                    "auth_code_123",
                    Some("state_123"),
                )?;

                assert_eq!(integration.id, 124);
                assert_eq!(integration.provider, "google");
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_integrations() -> Result<()> {
        let response = json!([
            {
                "id": 123,
                "provider": "google",
                "provider_user_id": "user@gmail.com",
                "linked_at": "2024-01-01T10:00:00Z",
                "updated_at": "2024-01-16T10:00:00Z"
            },
            {
                "id": 124,
                "provider": "outlook",
                "provider_user_id": "user@outlook.com",
                "linked_at": "2024-01-05T10:00:00Z",
                "updated_at": "2024-01-15T10:00:00Z"
            }
        ]);

        with_mockito(
            Method::GET,
            "/integrations/calendar",
            200,
            Some(response),
            |client| {
                let integrations = client.calendar().get_integrations()?;
                assert_eq!(integrations.len(), 2);
                assert_eq!(integrations[0].provider, "google");
                assert_eq!(integrations[1].provider, "outlook");
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_calendars() -> Result<()> {
        let response = json!({
            "calendars": [
                {
                    "id": "primary",
                    "integration_id": 123,
                    "name": "Primary Calendar",
                    "description": "Main calendar",
                    "selected": true,
                    "color": "#4285F4"
                },
                {
                    "id": "secondary",
                    "integration_id": 123,
                    "name": "Secondary Calendar",
                    "selected": false,
                    "color": "#DB4437"
                }
            ],
            "next_page_token": null
        });

        with_mockito(
            Method::GET,
            "/integrations/calendar/calendars",
            200,
            Some(response),
            |client| {
                let result = client.calendar().get_calendars()?;
                assert_eq!(result.calendars.len(), 2);
                assert_eq!(result.calendars[0].name, "Primary Calendar");
                assert!(result.calendars[0].selected);
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_selected_calendars() -> Result<()> {
        let response = json!([
            {
                "id": "primary",
                "integration_id": 123,
                "name": "Primary Calendar",
                "description": "Main calendar",
                "selected": true,
                "color": "#4285F4"
            },
            {
                "id": "work",
                "integration_id": 124,
                "name": "Work Calendar",
                "selected": true,
                "color": "#0F9D58"
            }
        ]);

        with_mockito(
            Method::GET,
            "/integrations/calendar/calendars/selected",
            200,
            Some(response),
            |client| {
                let calendars = client.calendar().get_selected_calendars()?;
                assert_eq!(calendars.len(), 2);
                assert!(calendars.iter().all(|c| c.selected));
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_events() -> Result<()> {
        let response = json!({
            "events": [
                {
                    "id": "evt1",
                    "calendar_id": "primary",
                    "integration_id": 123,
                    "title": "Morning Standup",
                    "description": "Daily team sync",
                    "start": "2024-01-16T09:00:00Z",
                    "end": "2024-01-16T09:30:00Z",
                    "all_day": false,
                    "recurring": true,
                    "location": "Zoom"
                },
                {
                    "id": "evt2",
                    "calendar_id": "primary",
                    "integration_id": 123,
                    "title": "Client Meeting",
                    "start": "2024-01-16T14:00:00Z",
                    "end": "2024-01-16T15:00:00Z",
                    "all_day": false,
                    "recurring": false
                }
            ],
            "next_page_token": null
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-16");
        params.insert("end_date", "2024-01-17");

        with_mockito_params(
            Method::GET,
            "/integrations/calendar/events",
            Some(params),
            200,
            Some(response),
            |client| {
                let result = client.calendar().get_events("2024-01-16", "2024-01-17")?;
                assert_eq!(result.events.len(), 2);
                assert_eq!(result.events[0].title, "Morning Standup");
                assert!(result.events[0].recurring);
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_event() -> Result<()> {
        let event = CalendarEvent {
            id: "evt1".to_string(),
            calendar_id: "primary".to_string(),
            integration_id: 123,
            title: "Updated Meeting".to_string(),
            description: Some("Updated description".to_string()),
            start: DateTime::parse_from_rfc3339("2024-01-16T14:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            end: DateTime::parse_from_rfc3339("2024-01-16T15:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            all_day: false,
            recurring: false,
            color: Some("#4285F4".to_string()),
            location: Some("Conference Room B".to_string()),
            attendees: Some(vec![
                "user1@example.com".to_string(),
                "user2@example.com".to_string(),
            ]),
        };

        let response = json!({
            "id": "evt1",
            "calendar_id": "primary",
            "integration_id": 123,
            "title": "Updated Meeting",
            "description": "Updated description",
            "start": "2024-01-16T14:00:00Z",
            "end": "2024-01-16T15:00:00Z",
            "all_day": false,
            "recurring": false,
            "location": "Conference Room B"
        });

        with_mockito(
            Method::POST,
            "/integrations/calendar/events/update",
            200,
            Some(response),
            |client| {
                let updated = client.calendar().update_event(&event)?;
                assert_eq!(updated.title, "Updated Meeting");
                assert_eq!(updated.location, Some("Conference Room B".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_setup_integration() -> Result<()> {
        let response = json!({
            "id": 125,
            "provider": "google",
            "provider_user_id": "newuser@gmail.com",
            "linked_at": "2024-01-16T12:00:00Z",
            "updated_at": "2024-01-16T12:00:00Z"
        });

        with_mockito(
            Method::POST,
            "/integrations/calendar/setup",
            200,
            Some(response),
            |client| {
                let integration = client.calendar().setup("google")?;
                assert_eq!(integration.id, 125);
                assert_eq!(integration.provider, "google");
                Ok(())
            },
        )
    }

    #[test]
    fn test_delete_integration() -> Result<()> {
        with_mockito(
            Method::DELETE,
            "/integrations/calendar/123",
            204,
            None,
            |client| {
                client.calendar().delete_integration(123)?;
                Ok(())
            },
        )
    }
}
