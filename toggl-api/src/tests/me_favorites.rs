#[cfg(test)]
mod tests {
    use crate::{models::api::favorite::*, tests::with_mockito};
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::Result;
    use toggl_core::{FavoriteId, ProjectId, TagId, WorkspaceId};

    #[test]
    fn test_get_favorites() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "workspace_id": 123,
                "user_id": 456,
                "description": "Favorite task",
                "project_id": 789,
                "task_id": null,
                "tag_ids": [1, 2],
                "billable": true,
                "created_at": "2023-01-01T10:00:00Z",
                "updated_at": "2023-01-01T10:00:00Z",
                "suggestion": false,
                "type_": "time_entry"
            }
        ]);

        with_mockito(
            Method::GET,
            "/me/favorites",
            200,
            Some(response),
            |client| {
                let favorites = client.me().get_favorites(None)?;
                assert_eq!(favorites.len(), 1);
                assert_eq!(favorites[0].id, FavoriteId(1));
                assert_eq!(favorites[0].description.as_ref().unwrap(), "Favorite task");
                Ok(())
            },
        )
    }

    #[test]
    fn test_create_favorite() -> Result<()> {
        let create_data = CreateFavorite {
            workspace_id: WorkspaceId(123),
            description: Some("New favorite".to_string()),
            project_id: Some(ProjectId(789)),
            task_id: None,
            tag_ids: Some(vec![TagId(1), TagId(2)]),
            billable: Some(true),
        };

        let response = json!({
            "id": 1,
            "workspace_id": 123,
            "user_id": 456,
            "description": "New favorite",
            "project_id": 789,
            "task_id": null,
            "tag_ids": [1, 2],
            "billable": true,
            "created_at": "2023-01-01T10:00:00Z",
            "updated_at": "2023-01-01T10:00:00Z",
            "suggestion": false,
            "type_": "time_entry"
        });

        with_mockito(
            Method::POST,
            "/me/favorites",
            200,
            Some(response),
            |client| {
                let favorite = client.me().create_favorite(&create_data)?;
                assert_eq!(favorite.id, FavoriteId(1));
                assert_eq!(favorite.description.as_ref().unwrap(), "New favorite");
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_favorite() -> Result<()> {
        let update_data = UpdateFavorite {
            description: Some("Updated favorite".to_string()),
            project_id: Some(ProjectId(999)),
            task_id: None,
            tag_ids: None,
            billable: Some(false),
        };

        let response = json!({
            "id": 1,
            "workspace_id": 123,
            "user_id": 456,
            "description": "Updated favorite",
            "project_id": 999,
            "task_id": null,
            "tag_ids": [1, 2],
            "billable": false,
            "created_at": "2023-01-01T10:00:00Z",
            "updated_at": "2023-01-01T11:00:00Z",
            "suggestion": false,
            "type_": "time_entry"
        });

        with_mockito(
            Method::PUT,
            "/me/favorites/1",
            200,
            Some(response),
            |client| {
                let favorite = client.me().update_favorite(FavoriteId(1), &update_data)?;
                assert_eq!(favorite.id, FavoriteId(1));
                assert_eq!(favorite.description.as_ref().unwrap(), "Updated favorite");
                assert!(!favorite.billable);
                Ok(())
            },
        )
    }

    #[test]
    fn test_delete_favorite() -> Result<()> {
        with_mockito(
            Method::DELETE,
            "/me/favorites/1",
            200,
            None::<serde_json::Value>,
            |client| {
                client.me().delete_favorite(FavoriteId(1))?;
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_favorite_suggestions() -> Result<()> {
        let response = json!([
            {
                "id": 2,
                "workspace_id": 123,
                "user_id": 456,
                "description": "Suggested task",
                "project_id": 789,
                "task_id": null,
                "tag_ids": [],
                "billable": false,
                "created_at": "2023-01-01T10:00:00Z",
                "updated_at": "2023-01-01T10:00:00Z",
                "suggestion": true,
                "type_": "time_entry"
            }
        ]);

        with_mockito(
            Method::GET,
            "/me/favorites/suggestions",
            200,
            Some(response),
            |client| {
                let suggestions = client.me().get_favorite_suggestions()?;
                assert_eq!(suggestions.len(), 1);
                assert_eq!(suggestions[0].id, FavoriteId(2));
                assert!(suggestions[0].suggestion);
                Ok(())
            },
        )
    }
}
