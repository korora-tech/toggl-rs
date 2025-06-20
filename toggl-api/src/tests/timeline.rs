use crate::models::api::ids::{ProjectId, TimelineId};
use crate::models::api::timeline::CreateTimelineEvent;
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;
use std::collections::BTreeMap;
use toggl_core::Result;

use super::{with_mockito, with_mockito_params};

#[test]
fn test_list_timeline_events() -> Result<()> {
    let response = json!([
        {
            "id": 1111,
            "start_time": "2024-01-01T09:00:00Z",
            "end_time": "2024-01-01T10:30:00Z",
            "duration": 5400,
            "description": "Morning standup and email",
            "activity_type": "computer",
            "project_id": null,
            "task_id": null
        },
        {
            "id": 2222,
            "start_time": "2024-01-01T10:30:00Z",
            "end_time": "2024-01-01T12:00:00Z",
            "duration": 5400,
            "description": "Development work",
            "activity_type": "computer",
            "project_id": 123456,
            "task_id": 789
        }
    ]);

    let mut params = BTreeMap::new();
    params.insert("start_date", "2024-01-01");
    params.insert("end_date", "2024-01-02");

    with_mockito_params(
        Method::GET,
        "/timeline",
        Some(params),
        200,
        Some(response),
        |client| {
            let events = client.timeline().list("2024-01-01", "2024-01-02")?;

            assert_eq!(2, events.len());
            assert_eq!(
                "Morning standup and email",
                events[0].description.as_ref().unwrap()
            );
            assert_eq!(5400, events[0].duration);
            assert!(events[0].project_id.is_none());
            assert_eq!(Some(ProjectId::new(123456)), events[1].project_id);
            Ok(())
        },
    )
}

#[test]
fn test_create_timeline_event() -> Result<()> {
    let create_event = CreateTimelineEvent {
        start_time: "2024-01-01T14:00:00Z".to_string(),
        end_time: Some("2024-01-01T15:00:00Z".to_string()),
        description: Some("Meeting with client".to_string()),
        activity_type: "meeting".to_string(),
        project_id: Some(ProjectId::new(123456)),
        task_id: None,
    };

    let response = json!({
        "id": 3333,
        "start_time": "2024-01-01T14:00:00Z",
        "end_time": "2024-01-01T15:00:00Z",
        "duration": 3600,
        "description": "Meeting with client",
        "activity_type": "meeting",
        "project_id": 123456,
        "task_id": null
    });

    with_mockito(Method::POST, "/timeline", 200, Some(response), |client| {
        let event = client.timeline().create(&create_event)?;
        assert_eq!(TimelineId::new(3333), event.id);
        assert_eq!("Meeting with client", event.description.as_ref().unwrap());
        assert_eq!(3600, event.duration);
        assert_eq!(Some(ProjectId::new(123456)), event.project_id);
        Ok(())
    })
}

#[test]
fn test_delete_timeline_event() -> Result<()> {
    with_mockito(Method::DELETE, "/timeline/3333", 200, None, |client| {
        client.timeline().delete(TimelineId::new(3333))?;
        Ok(())
    })
}
