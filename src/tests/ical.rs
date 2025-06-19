use crate::error::Result;
use crate::tests::with_mockito;
use reqwest::Method;
use serde_json::json;

#[test]
fn test_get_ical_file() -> Result<()> {
    let ical_content = r#"BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//Toggl//Toggl Track//EN
CALSCALE:GREGORIAN
METHOD:PUBLISH
X-WR-CALNAME:Toggl Track
X-WR-CALDESC:Toggl Track Calendar
BEGIN:VEVENT
DTSTART:20240115T100000Z
DTEND:20240115T110000Z
SUMMARY:Code review
DESCRIPTION:Project: My Project
UID:time-entry-999@toggl.com
END:VEVENT
END:VCALENDAR"#;

    with_mockito(
        Method::GET,
        "/ical/workspace_user/test-token-123",
        200,
        Some(json!(ical_content)),
        |client| {
            let content = client.ical().get_ical_file("test-token-123")?;
            assert!(content.contains("BEGIN:VCALENDAR"));
            assert!(content.contains("Code review"));
            Ok(())
        },
    )
}

#[test]
fn test_reset_ical_token() -> Result<()> {
    let new_token = "new-token-456";

    with_mockito(
        Method::POST,
        "/workspaces/987654/ical/reset",
        200,
        Some(json!(new_token)),
        |client| {
            let token = client.ical().reset_token(987654)?;
            assert_eq!(token, "new-token-456");
            Ok(())
        },
    )
}

#[test]
fn test_toggle_ical() -> Result<()> {
    let status_message = "iCal feed enabled";

    with_mockito(
        Method::POST,
        "/workspaces/987654/ical/toggle",
        200,
        Some(json!(status_message)),
        |client| {
            let status = client.ical().toggle_ical(987654)?;
            assert_eq!(status, "iCal feed enabled");
            Ok(())
        },
    )
}
