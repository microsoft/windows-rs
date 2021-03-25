use test_wildcard::Windows::Foundation::Uri;

#[test]
fn test() -> windows::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;
    let _ = uri.QueryParsed()?;

    Ok(())
}
