use crate::error::Result;
use crate::model::api::favorite::{CreateFavorite, UpdateFavorite};
use crate::tests::with_mockito;
use reqwest::Method;
use serde_json::json;

#[test]
fn test_get_workspace_favorites() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "workspace_id": 12345,
            "user_id": 6789,
            "description": "Writing reports",
            "project_id": 111,
            "task_id": 222,
            "tag_ids": [333, 444],
            "billable": true,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z",
            "suggestion": false,
            "type_": "time_entry"
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/12345/favorites",
        200,
        Some(response),
        |client| {
            let favorites = client.workspaces().get_favorites(12345, None)?;
            assert_eq!(favorites.len(), 1);
            assert_eq!(favorites[0].id, 1);
            assert_eq!(
                favorites[0].description,
                Some("Writing reports".to_string())
            );
            assert_eq!(favorites[0].project_id, Some(111));
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_favorites_with_since() -> Result<()> {
    let response = json!([]);

    with_mockito(
        Method::GET,
        "/workspaces/12345/favorites?since=1704067200",
        200,
        Some(response),
        |client| {
            let favorites = client.workspaces().get_favorites(12345, Some(1704067200))?;
            assert_eq!(favorites.len(), 0);
            Ok(())
        },
    )
}

#[test]
fn test_create_workspace_favorite() -> Result<()> {
    let response = json!({
        "id": 1,
        "workspace_id": 12345,
        "user_id": 6789,
        "description": "New favorite",
        "project_id": 111,
        "task_id": null,
        "tag_ids": [],
        "billable": false,
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z",
        "suggestion": false,
        "type_": "time_entry"
    });

    with_mockito(
        Method::POST,
        "/workspaces/12345/favorites",
        200,
        Some(response),
        |client| {
            let new_favorite = CreateFavorite {
                workspace_id: 12345,
                description: Some("New favorite".to_string()),
                project_id: Some(111),
                task_id: None,
                tag_ids: None,
                billable: Some(false),
            };
            let favorite = client.workspaces().create_favorite(12345, &new_favorite)?;
            assert_eq!(favorite.id, 1);
            assert_eq!(favorite.description, Some("New favorite".to_string()));
            Ok(())
        },
    )
}

#[test]
fn test_update_workspace_favorites() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "workspace_id": 12345,
            "user_id": 6789,
            "description": "Updated favorite",
            "project_id": 111,
            "task_id": null,
            "tag_ids": [],
            "billable": true,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z",
            "suggestion": false,
            "type_": "time_entry"
        }
    ]);

    with_mockito(
        Method::PUT,
        "/workspaces/12345/favorites",
        200,
        Some(response),
        |client| {
            let updates = vec![UpdateFavorite {
                description: Some("Updated favorite".to_string()),
                project_id: Some(111),
                task_id: None,
                tag_ids: None,
                billable: Some(true),
            }];
            let favorites = client.workspaces().update_favorites(12345, &updates)?;
            assert_eq!(favorites.len(), 1);
            assert_eq!(
                favorites[0].description,
                Some("Updated favorite".to_string())
            );
            assert!(favorites[0].billable);
            Ok(())
        },
    )
}

#[test]
fn test_delete_workspace_favorite() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/12345/favorites/1",
        200,
        None,
        |client| {
            client.workspaces().delete_favorite(12345, 1)?;
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_favorite_suggestions() -> Result<()> {
    let response = json!([
        {
            "id": 2,
            "workspace_id": 12345,
            "user_id": 6789,
            "description": "Suggested favorite",
            "project_id": 333,
            "task_id": null,
            "tag_ids": [],
            "billable": false,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "suggestion": true,
            "type_": "time_entry"
        }
    ]);

    with_mockito(
        Method::POST,
        "/workspaces/12345/favorites/suggestions",
        200,
        Some(response),
        |client| {
            let suggestions = client.workspaces().get_favorite_suggestions(12345)?;
            assert_eq!(suggestions.len(), 1);
            assert!(suggestions[0].suggestion);
            assert_eq!(
                suggestions[0].description,
                Some("Suggested favorite".to_string())
            );
            Ok(())
        },
    )
}
