winrt::import!(
    dependencies
        os
    modules
        "windows.foundation"
);

#[test]
fn try_into() -> winrt::Result<()> {
    use winrt::ComInterface;
    use winrt::TryInto;

    use windows::foundation::IClosable;
    use windows::foundation::IStringable;
    use windows::foundation::Uri;

    let uri = Uri::create_uri("http://kennykerr.ca")?;

    // Uri implements IStringable so this query should succeed.
    let s: IStringable = uri.try_into().unwrap();
    assert!(s.to_string()? == "http://kennykerr.ca/");

    // Uri does not implement IClosable so this should fail.
    let c: winrt::Result<IClosable> = uri.try_into();
    assert!(c.is_err());

    // And we should be able to query an interface for a class and it should use
    // its default interface GUID to resolve the query.
    let uri: Uri = s.try_into().unwrap();
    assert!(uri.domain()? == "kennykerr.ca");

    // Given a null Uri...
    let uri = Uri::default();
    assert!(uri.is_null());

    // ...the try_into succeeds, but returns a null IStringable.
    let s: IStringable = uri.try_into().unwrap();
    assert!(s.is_null());

    Ok(())
}
