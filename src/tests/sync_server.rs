use crate::error::Result;
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;

use super::with_mockito;

#[test]
fn test_get_goals() -> Result<()> {
    let response = json!([
        {
            "id": 12345,
            "workspace_id": 1234567,
            "name": "Q1 Development Goals",
            "start_date": "2024-01-01",
            "end_date": "2024-03-31",
            "duration": 432000,
            "project_ids": [111111, 222222, 333333],
            "active": true
        },
        {
            "id": 67890,
            "workspace_id": 1234567,
            "name": "Marketing Campaign",
            "start_date": "2024-02-01",
            "end_date": "2024-02-29",
            "duration": 144000,
            "project_ids": [444444],
            "active": true
        }
    ]);

    with_mockito(
        Method::GET,
        "/sync-server/me/goals",
        200,
        Some(response),
        |client| {
            let goals = client.sync_server().get_goals()?;
            assert_eq!(2, goals.len());
            assert_eq!("Q1 Development Goals", goals[0].name);
            assert_eq!(432000, goals[0].duration);
            assert_eq!(3, goals[0].project_ids.len());
            assert_eq!("Marketing Campaign", goals[1].name);
            assert_eq!(144000, goals[1].duration);
            assert_eq!(true, goals[1].active);
            Ok(())
        },
    )
}
