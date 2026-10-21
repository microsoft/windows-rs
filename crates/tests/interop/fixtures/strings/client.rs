use super::*;
use Strings::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;

    let empty = HSTRING::new();
    assert_eq!(widget.Length(&empty)?, 0);
    assert!(widget.Echo(&empty)?.is_empty());

    let embedded = HSTRING::from("a\0b");
    assert_eq!(widget.Length(&embedded)?, 3);
    assert_eq!(widget.Echo(&embedded)?, embedded);

    let pair = HSTRING::from("😀");
    assert_eq!(widget.Length(&pair)?, 2);
    assert_eq!(widget.Echo(&pair)?, pair);

    let unpaired = HSTRING::from_wide(&[0xd800]);
    assert_eq!(widget.Length(&unpaired)?, 1);
    assert_eq!(widget.Echo(&unpaired)?, unpaired);

    let large = widget.Repeat(h!("ab"), 100_000)?;
    assert_eq!(large.len(), 200_000);
    assert_eq!(large, "ab".repeat(100_000));
    assert_eq!(widget.Echo(&large)?, large);

    let greeting = widget.Greeting()?;
    assert_eq!(greeting, "héllo 😀");
    assert_eq!(greeting.len(), 8);

    assert!(widget.Text()?.is_empty());
    widget.SetText(&embedded)?;
    assert_eq!(widget.Text()?, embedded);
    widget.SetText(&empty)?;
    assert!(widget.Text()?.is_empty());

    Ok(())
}
