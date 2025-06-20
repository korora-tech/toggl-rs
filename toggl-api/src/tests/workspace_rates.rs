use crate::models::api::rates::{CreateRate, RateLevel, RateMode};
use crate::tests::with_mockito;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;
use toggl_core::{LevelId, ProjectId, WorkspaceId, WorkspaceUserId};

#[test]
fn test_create_workspace_rate() -> Result<()> {
    with_mockito(
        Method::POST,
        "/workspaces/12345/rates",
        201,
        None,
        |client| {
            let new_rate = CreateRate {
                amount: 150.0,
                level: RateLevel::Project,
                level_id: LevelId::new(555),
                rate_type: "billable_rates".to_string(),
                mode: Some(RateMode::StartToday),
                start: None,
            };
            client
                .workspaces()
                .create_rate(WorkspaceId(12345), &new_rate)?;
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_rates() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "amount": 150.0,
            "workspace_id": 12345,
            "project_id": 555,
            "type": "billable_rates",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "start": "2024-01-01T00:00:00Z"
        },
        {
            "id": 2,
            "amount": 200.0,
            "workspace_id": 12345,
            "project_id": 555,
            "type": "billable_rates",
            "created_at": "2024-01-02T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z",
            "start": "2024-01-02T00:00:00Z"
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/12345/rates/project/555",
        200,
        Some(response),
        |client| {
            let rates = client.workspaces().get_rates(
                WorkspaceId(12345),
                &RateLevel::Project,
                555,
                None,
            )?;
            assert_eq!(rates.len(), 2);
            assert_eq!(rates[0].amount, Some(150.0));
            assert_eq!(rates[0].project_id, Some(ProjectId::new(555)));
            assert_eq!(rates[1].amount, Some(200.0));
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_rates_with_type() -> Result<()> {
    let response = json!([
        {
            "id": 3,
            "amount": 100.0,
            "workspace_id": 12345,
            "workspace_user_id": 777,
            "type": "labor_costs",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "start": "2024-01-01T00:00:00Z"
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/12345/rates/workspace_user/777?type=labor_costs",
        200,
        Some(response),
        |client| {
            let rates = client.workspaces().get_rates(
                WorkspaceId(12345),
                &RateLevel::WorkspaceUser,
                777,
                Some("labor_costs"),
            )?;
            assert_eq!(rates.len(), 1);
            assert_eq!(rates[0].amount, Some(100.0));
            assert_eq!(rates[0].workspace_user_id, Some(WorkspaceUserId::new(777)));
            assert_eq!(rates[0].rate_type, Some("labor_costs".to_string()));
            Ok(())
        },
    )
}
