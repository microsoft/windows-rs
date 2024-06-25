use windows_strings::*;

#[test]
fn bstr() -> Result<()> {
    let s = BSTR::from("hello");
    assert_eq!(s.len(), 5);

    Ok(())
}
