use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

use super::with_mockito;

#[test]
fn test_list_timezones() -> Result<()> {
    let response = json!([
        {
            "name": "America/New_York",
            "offset": -18000
        },
        {
            "name": "Europe/London",
            "offset": 0
        },
        {
            "name": "Asia/Tokyo",
            "offset": 32400
        }
    ]);

    with_mockito(Method::GET, "/timezones", 200, Some(response), |client| {
        let timezones = client.timezones().list()?;
        assert_eq!(3, timezones.len());
        assert_eq!("America/New_York", timezones[0].name);
        assert_eq!(0, timezones[1].offset);
        assert_eq!(32400, timezones[2].offset);
        Ok(())
    })
}

#[test]
fn test_get_timezone_offsets() -> Result<()> {
    let response = json!([
        {
            "timezone": "America/New_York",
            "offset": -18000,
            "offset_name": "EST"
        },
        {
            "timezone": "Europe/London",
            "offset": 0,
            "offset_name": "GMT"
        }
    ]);

    with_mockito(
        Method::GET,
        "/timezones/offsets",
        200,
        Some(response),
        |client| {
            let offsets = client.timezones().get_offsets()?;
            assert_eq!(2, offsets.len());
            assert_eq!("America/New_York", offsets[0].timezone);
            assert_eq!("GMT", offsets[1].offset_name);
            Ok(())
        },
    )
}
