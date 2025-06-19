use crate::error::Result;
use crate::model::api::smail::{ContactRequest, DemoRequest, MeetRequest};
use reqwest::Method;

use super::with_mockito;

#[test]
fn test_send_contact_request() -> Result<()> {
    let contact_request = ContactRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        company: Some("Acme Corp".to_string()),
        company_size: Some("50-100".to_string()),
        message: "I'd like to learn more about Toggl Track".to_string(),
    };

    with_mockito(Method::POST, "/smail/contact", 200, None, |client| {
        client.smail().contact(&contact_request)?;
        Ok(())
    })
}

#[test]
fn test_request_demo() -> Result<()> {
    let demo_request = DemoRequest {
        full_name: "Jane Smith".to_string(),
        email: "jane@example.com".to_string(),
        company_name: "Tech Corp".to_string(),
        company_size: "100-500".to_string(),
        phone_number: Some("+1234567890".to_string()),
        country: "United States".to_string(),
        additional_info: Some("Looking for enterprise features".to_string()),
    };

    with_mockito(Method::POST, "/smail/demo", 200, None, |client| {
        client.smail().demo(&demo_request)?;
        Ok(())
    })
}

#[test]
fn test_schedule_meeting() -> Result<()> {
    let meet_request = MeetRequest {
        full_name: "Bob Johnson".to_string(),
        email: "bob@example.com".to_string(),
        date: "2024-01-15".to_string(),
        time: "14:00".to_string(),
        timezone: "America/New_York".to_string(),
        notes: Some("Interested in team features".to_string()),
    };

    with_mockito(Method::POST, "/smail/meet", 200, None, |client| {
        client.smail().meet(&meet_request)?;
        Ok(())
    })
}
