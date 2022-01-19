use test_build_wildcard::Windows::Foundation::Uri;

#[test]
fn test() -> windows::core::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;
    let _ = uri.QueryParsed()?;

    Ok(())
}
