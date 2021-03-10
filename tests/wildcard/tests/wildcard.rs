use test_wildcard::windows::foundation::Uri;

#[test]
fn test() -> windows::Result<()> {
    let uri = Uri::create_uri("http://kennykerr.ca")?;
    let _ = uri.query_parsed()?;

    Ok(())
}
