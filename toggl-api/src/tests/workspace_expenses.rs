use crate::client::TogglClient;
use crate::models::api::expense::CreateExpense;
use crate::tests::*;
use chrono::NaiveDate;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;
use toggl_core::{CategoryId, ExpenseId, ProjectId, WorkspaceId};

#[test]
fn test_get_expenses() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let response = json!([
        {
            "id": 1001,
            "workspace_id": 12345,
            "user_id": 98765,
            "project_id": 54321,
            "task_id": null,
            "spent_at": "2024-01-15",
            "description": "Client lunch meeting",
            "amount": 85.50,
            "currency": "USD",
            "category_id": 1,
            "category_name": "Meals",
            "billable": true,
            "payee": null,
            "receipt_url": null,
            "created_at": "2024-01-15T10:00:00Z",
            "updated_at": "2024-01-15T10:00:00Z",
            "at": "2024-01-15T10:00:00Z"
        },
        {
            "id": 1002,
            "workspace_id": 12345,
            "user_id": 98765,
            "project_id": 54322,
            "task_id": null,
            "spent_at": "2024-01-16",
            "description": "Travel expenses",
            "amount": 250.00,
            "currency": "USD",
            "category_id": 2,
            "category_name": "Transportation",
            "billable": true,
            "payee": "Taxi Company",
            "receipt_url": null,
            "created_at": "2024-01-16T14:00:00Z",
            "updated_at": "2024-01-16T14:00:00Z",
            "at": "2024-01-16T14:00:00Z"
        }
    ]);

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/expenses", workspace_id),
        200,
        Some(response),
        |client| {
            let expenses = client.workspaces().get_expenses(workspace_id)?;
            assert_eq!(expenses.len(), 2);
            assert_eq!(expenses[0].description, "Client lunch meeting");
            assert_eq!(expenses[0].amount, 85.50);
            assert_eq!(expenses[0].category_name, "Meals");
            assert!(expenses[0].billable);
            assert_eq!(expenses[1].description, "Travel expenses");
            assert_eq!(expenses[1].amount, 250.00);
            assert_eq!(expenses[1].payee, Some("Taxi Company".to_string()));
            Ok(())
        },
    )
}

#[test]
fn test_create_expense() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let new_expense = CreateExpense {
        workspace_id,
        spent_at: NaiveDate::parse_from_str("2024-01-17", "%Y-%m-%d").unwrap(),
        description: "Software subscription".to_string(),
        amount: 49.99,
        currency: "USD".to_string(),
        category_id: CategoryId(3),
        project_id: Some(ProjectId(54321)),
        task_id: None,
        billable: Some(true),
        payee: None,
    };

    let response = json!({
        "id": 1003,
        "workspace_id": 12345,
        "user_id": 98765,
        "project_id": 54321,
        "task_id": null,
        "spent_at": "2024-01-17",
        "description": "Software subscription",
        "amount": 49.99,
        "currency": "USD",
        "category_id": 3,
        "category_name": "Software",
        "billable": true,
        "payee": null,
        "receipt_url": null,
        "created_at": "2024-01-17T09:30:00Z",
        "updated_at": "2024-01-17T09:30:00Z",
        "at": "2024-01-17T09:30:00Z"
    });

    with_mockito(
        Method::POST,
        &format!("/workspaces/{}/expenses", workspace_id),
        201,
        Some(response),
        |client| {
            let expense = client
                .workspaces()
                .create_expense(workspace_id, &new_expense)?;
            assert_eq!(expense.id, ExpenseId(1003));
            assert_eq!(expense.description, "Software subscription");
            assert_eq!(expense.amount, 49.99);
            assert_eq!(expense.category_name, "Software");
            assert!(expense.billable);
            Ok(())
        },
    )
}

#[test]
fn test_get_empty_expenses() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let response = json!([]);

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/expenses", workspace_id),
        200,
        Some(response),
        |client| {
            let expenses = client.workspaces().get_expenses(workspace_id)?;
            assert!(expenses.is_empty());
            Ok(())
        },
    )
}

#[test]
fn test_create_minimal_expense() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let new_expense = CreateExpense {
        workspace_id,
        spent_at: NaiveDate::parse_from_str("2024-01-18", "%Y-%m-%d").unwrap(),
        description: "Office supplies".to_string(),
        amount: 25.00,
        currency: "USD".to_string(),
        category_id: CategoryId(4),
        project_id: None,
        task_id: None,
        billable: None,
        payee: None,
    };

    let response = json!({
        "id": 1004,
        "workspace_id": 12345,
        "user_id": 98765,
        "project_id": null,
        "task_id": null,
        "spent_at": "2024-01-18",
        "description": "Office supplies",
        "amount": 25.00,
        "currency": "USD",
        "category_id": 4,
        "category_name": "Other",
        "billable": false,
        "payee": null,
        "receipt_url": null,
        "created_at": "2024-01-18T11:00:00Z",
        "updated_at": "2024-01-18T11:00:00Z",
        "at": "2024-01-18T11:00:00Z"
    });

    with_mockito(
        Method::POST,
        &format!("/workspaces/{}/expenses", workspace_id),
        201,
        Some(response),
        |client| {
            let expense = client
                .workspaces()
                .create_expense(workspace_id, &new_expense)?;
            assert_eq!(expense.id, ExpenseId(1004));
            assert_eq!(expense.description, "Office supplies");
            assert_eq!(expense.project_id, None);
            assert!(!expense.billable);
            Ok(())
        },
    )
}

#[test]
fn test_upload_expense() -> Result<()> {
    let workspace_id = WorkspaceId(12345);
    let expense_data =
        b"date,description,amount,currency,category_id\n2024-01-20,Test expense,100.00,USD,1";

    // Create custom mock for multipart upload
    let mut server = mockito::Server::new();
    let mock = server
        .mock(
            "POST",
            format!("/workspaces/{}/expenses/upload", workspace_id).as_str(),
        )
        .match_header("authorization", mockito::Matcher::Any)
        .match_header(
            "content-type",
            mockito::Matcher::Regex("multipart/form-data; boundary=.*".to_string()),
        )
        .with_status(204)
        .create();

    let toggl_client = TogglClient::new_with_base_url("test_api_token".to_string(), &server.url())?;

    let result = toggl_client
        .workspaces()
        .upload_expense(workspace_id, expense_data);

    assert!(result.is_ok());
    mock.assert();

    Ok(())
}
