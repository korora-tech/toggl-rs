use crate::error::Result;
use crate::model::api::feedback::{FeedbackRequest, WebFeedbackRequest};
use reqwest::Method;

use super::with_mockito;

#[test]
fn test_send_feedback() -> Result<()> {
    with_mockito(Method::POST, "/feedback", 200, None, |client| {
        let feedback_req = FeedbackRequest {
            subject: "Bug Report".to_string(),
            details: "The timer is not working properly".to_string(),
            toggl_version: Some("8.5.0".to_string()),
            os: Some("macOS".to_string()),
            os_version: Some("13.0".to_string()),
            device: Some("MacBook Pro".to_string()),
        };
        client.feedback().send(&feedback_req)?;
        Ok(())
    })
}

#[test]
fn test_send_web_feedback() -> Result<()> {
    with_mockito(Method::POST, "/feedback/web", 200, None, |client| {
        let feedback_req = WebFeedbackRequest {
            message: "The website is slow".to_string(),
            browser: Some("Chrome".to_string()),
            browser_version: Some("108.0".to_string()),
            os: Some("Windows".to_string()),
            url: Some("https://track.toggl.com/timer".to_string()),
        };
        client.feedback().send_web(&feedback_req)?;
        Ok(())
    })
}

#[test]
fn test_send_mobile_feedback() -> Result<()> {
    with_mockito(Method::POST, "/mobile/feedback", 200, None, |client| {
        let feedback_req = FeedbackRequest {
            subject: "Feature Request".to_string(),
            details: "Add dark mode support".to_string(),
            toggl_version: Some("3.2.1".to_string()),
            os: Some("iOS".to_string()),
            os_version: Some("16.0".to_string()),
            device: Some("iPhone 14".to_string()),
        };
        client.feedback().send_mobile(&feedback_req)?;
        Ok(())
    })
}
