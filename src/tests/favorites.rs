#[cfg(test)]
mod tests {
    use crate::model::api::favorite::*;
    use crate::tests::*;
    use reqwest::Method;
    use serde_json::json;

    #[test]
    fn test_list_favorites() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "workspace_id": 123456,
                "user_id": 789012,
                "description": "Task 1",
                "project_id": 111,
                "task_id": null,
                "tag_ids": [1, 2],
                "billable": true,
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
                "suggestion": false,
                "type_": "time_entry"
            },
            {
                "id": 2,
                "workspace_id": 123456,
                "user_id": 789012,
                "description": "Task 2",
                "project_id": 222,
                "task_id": 333,
                "tag_ids": [],
                "billable": false,
                "created_at": "2024-01-02T00:00:00Z",
                "updated_at": "2024-01-02T00:00:00Z",
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
                let favorites = client.favorites().list()?;

                assert_eq!(favorites.len(), 2);
                assert_eq!(favorites[0].id, 1);
                assert_eq!(favorites[0].description, Some("Task 1".to_string()));
                assert_eq!(favorites[0].project_id, Some(111));
                assert!(favorites[0].billable);

                assert_eq!(favorites[1].id, 2);
                assert_eq!(favorites[1].description, Some("Task 2".to_string()));
                assert_eq!(favorites[1].project_id, Some(222));
                assert_eq!(favorites[1].task_id, Some(333));
                assert!(!favorites[1].billable);

                Ok(())
            },
        )
    }

    #[test]
    fn test_create_favorite() -> Result<()> {
        let response = json!({
            "id": 123,
            "workspace_id": 123456,
            "user_id": 789012,
            "description": "New favorite",
            "project_id": 111,
            "task_id": null,
            "tag_ids": [1, 2],
            "billable": true,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "suggestion": false,
            "type_": "time_entry"
        });

        with_mockito(
            Method::POST,
            "/me/favorites",
            200,
            Some(response),
            |client| {
                let new_favorite = CreateFavorite {
                    workspace_id: 123456,
                    description: Some("New favorite".to_string()),
                    project_id: Some(111),
                    task_id: None,
                    tag_ids: Some(vec![1, 2]),
                    billable: Some(true),
                };

                let favorite = client.favorites().create(&new_favorite)?;

                assert_eq!(favorite.id, 123);
                assert_eq!(favorite.description, Some("New favorite".to_string()));
                assert_eq!(favorite.project_id, Some(111));
                assert!(favorite.billable);
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_favorite() -> Result<()> {
        let response = json!({
            "id": 123,
            "workspace_id": 123456,
            "user_id": 789012,
            "description": "Updated favorite",
            "project_id": 222,
            "task_id": null,
            "tag_ids": [3, 4],
            "billable": false,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z",
            "suggestion": false,
            "type_": "time_entry"
        });

        with_mockito(
            Method::PUT,
            "/me/favorites/123",
            200,
            Some(response),
            |client| {
                let update_favorite = UpdateFavorite {
                    description: Some("Updated favorite".to_string()),
                    project_id: Some(222),
                    task_id: None,
                    tag_ids: Some(vec![3, 4]),
                    billable: Some(false),
                };

                let favorite = client.favorites().update(123, &update_favorite)?;

                assert_eq!(favorite.id, 123);
                assert_eq!(favorite.description, Some("Updated favorite".to_string()));
                assert_eq!(favorite.project_id, Some(222));
                assert!(!favorite.billable);
                Ok(())
            },
        )
    }

    #[test]
    fn test_delete_favorite() -> Result<()> {
        with_mockito(Method::DELETE, "/me/favorites/123", 200, None, |client| {
            client.favorites().delete(123)?;
            Ok(())
        })
    }

    #[test]
    fn test_get_suggestions() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "workspace_id": 123456,
                "user_id": 789012,
                "description": "Suggested task 1",
                "project_id": 111,
                "task_id": null,
                "tag_ids": [],
                "billable": true,
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
                "suggestion": true,
                "type_": "time_entry"
            },
            {
                "id": 2,
                "workspace_id": 123456,
                "user_id": 789012,
                "description": "Suggested task 2",
                "project_id": 222,
                "task_id": null,
                "tag_ids": [],
                "billable": false,
                "created_at": "2024-01-02T00:00:00Z",
                "updated_at": "2024-01-02T00:00:00Z",
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
                let suggestions = client.favorites().get_suggestions()?;

                assert_eq!(suggestions.len(), 2);
                assert_eq!(
                    suggestions[0].description,
                    Some("Suggested task 1".to_string())
                );
                assert!(suggestions[0].suggestion);
                assert_eq!(
                    suggestions[1].description,
                    Some("Suggested task 2".to_string())
                );
                assert!(suggestions[1].suggestion);
                Ok(())
            },
        )
    }
}
