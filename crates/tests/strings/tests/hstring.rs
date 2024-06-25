use windows_strings::*;

#[test]
fn hstring() -> Result<()> {
    let s = HSTRING::from("hello");
    assert_eq!(s.len(), 5);

    Ok(())
}
