use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

use super::with_mockito;

#[test]
fn test_get_avatars() -> Result<()> {
    let response = json!({
        "url": "https://avatars.toggl.com/123.jpg",
        "url_large": "https://avatars.toggl.com/123_large.jpg"
    });

    with_mockito(Method::GET, "/avatars", 200, Some(response), |client| {
        let avatar = client.avatars().get()?;
        assert_eq!("https://avatars.toggl.com/123.jpg", avatar.url);
        assert_eq!("https://avatars.toggl.com/123_large.jpg", avatar.url_large);
        Ok(())
    })
}

#[test]
fn test_use_gravatar() -> Result<()> {
    let response = json!({
        "url": "https://www.gravatar.com/avatar/abc?s=100",
        "url_large": "https://www.gravatar.com/avatar/abc?s=200"
    });

    // First mock the POST request
    let mut server = mockito::Server::new();
    let mock_post = server
        .mock("POST", "/avatars/use_gravatar")
        .with_status(200)
        .create();

    // Then mock the GET request that follows
    let mock_get = server
        .mock("GET", "/avatars")
        .with_status(200)
        .with_body(response.to_string())
        .create();

    let client =
        crate::client::TogglClient::new_with_base_url("test_api_token".to_string(), &server.url())?;

    let avatar = client.avatars().use_gravatar()?;
    assert!(avatar.url.contains("gravatar.com"));

    mock_post.assert();
    mock_get.assert();

    Ok(())
}
