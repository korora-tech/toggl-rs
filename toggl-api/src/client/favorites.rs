use super::TogglClient;
use crate::models::api::favorite::*;
use reqwest::Method;
use toggl_core::Result;

pub struct FavoritesClient {
    client: TogglClient,
}

impl FavoritesClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get favorites
    pub fn list(&self) -> Result<Vec<Favorite>> {
        self.client.request(Method::GET, "me/favorites")
    }

    /// Create favorite
    pub fn create(&self, favorite: &CreateFavorite) -> Result<Favorite> {
        self.client
            .request_with_body(Method::POST, "me/favorites", favorite)
    }

    /// Update favorite
    pub fn update(&self, favorite_id: u64, favorite: &UpdateFavorite) -> Result<Favorite> {
        self.client.request_with_body(
            Method::PUT,
            &format!("me/favorites/{}", favorite_id),
            favorite,
        )
    }

    /// Delete favorite
    pub fn delete(&self, favorite_id: u64) -> Result<()> {
        self.client
            .request_empty(Method::DELETE, &format!("me/favorites/{}", favorite_id))
    }

    /// Get favorite suggestions
    pub fn get_suggestions(&self) -> Result<Vec<Favorite>> {
        self.client.request(Method::GET, "me/favorites/suggestions")
    }
}
