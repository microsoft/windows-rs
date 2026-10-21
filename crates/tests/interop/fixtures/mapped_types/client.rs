use super::*;
use MappedTypes::*;
use bindings::Windows::Foundation::Uri;
use windows_time::{DateTime, TimeSpan};

// 2000-01-01 00:00:00 UTC.
const Y2K: i64 = 125_911_584_000_000_000;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    let time = DateTime::from_ticks(Y2K);
    let span = TimeSpan::from_minutes(90);
    assert_eq!(widget.Ticks(time)?, Y2K);
    assert_eq!(widget.Duration(span)?, 90 * 60 * TimeSpan::TICKS_PER_SECOND);
    assert_eq!(widget.Add(time, span)?, time + span);

    let uri = Uri::CreateUri(h!("https://example.com/path"))?;
    assert_eq!(widget.Host(&uri)?, "example.com");
    assert_eq!(
        widget.CreateUri(h!("https://example.com/path"))?.Host()?,
        "example.com"
    );

    assert_eq!(widget.ToString()?, "widget");
    let closed = Widget::Closed()?;
    widget.Close()?;
    assert_eq!(Widget::Closed()?, closed + 1);

    Ok(())
}
