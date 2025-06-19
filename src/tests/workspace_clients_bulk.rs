use crate::error::Result;
use crate::tests::with_mockito;
use reqwest::Method;
use serde_json::json;

#[test]
fn test_archive_clients_bulk() -> Result<()> {
    let response = json!({
        "client_ids": [123, 456, 789],
        "project_ids": [111, 222, 333]
    });

    with_mockito(
        Method::POST,
        "/workspaces/12345/clients/archive",
        200,
        Some(response),
        |client| {
            let client_ids = vec![123, 456, 789];
            let result = client
                .workspaces()
                .archive_clients_bulk(12345, client_ids)?;

            assert_eq!(result.client_ids, Some(vec![123, 456, 789]));
            assert_eq!(result.project_ids, Some(vec![111, 222, 333]));
            Ok(())
        },
    )
}

#[test]
fn test_get_clients_data() -> Result<()> {
    let response = json!([
        {
            "id": 123,
            "workspace_id": 12345,
            "name": "Client A",
            "archived": false,
            "at": "2024-01-01T00:00:00Z",
            "server_deleted_at": null,
            "permissions": ["update", "delete"]
        },
        {
            "id": 456,
            "workspace_id": 12345,
            "name": "Client B",
            "archived": true,
            "at": "2024-01-02T00:00:00Z",
            "server_deleted_at": null,
            "permissions": ["update"]
        }
    ]);

    with_mockito(
        Method::POST,
        "/workspaces/12345/clients/data",
        200,
        Some(response),
        |client| {
            let client_ids = vec![123, 456];
            let clients = client.workspaces().get_clients_data(12345, client_ids)?;

            assert_eq!(clients.len(), 2);
            assert_eq!(clients[0].id, 123);
            assert_eq!(clients[0].name, "Client A");
            assert!(!clients[0].archived);
            assert_eq!(clients[1].id, 456);
            assert_eq!(clients[1].name, "Client B");
            assert!(clients[1].archived);
            Ok(())
        },
    )
}

#[test]
fn test_delete_clients_bulk() -> Result<()> {
    with_mockito(
        Method::POST,
        "/workspaces/12345/clients/delete",
        200,
        None,
        |client| {
            let client_ids = vec![123, 456, 789];
            client.workspaces().delete_clients_bulk(12345, client_ids)?;
            Ok(())
        },
    )
}
