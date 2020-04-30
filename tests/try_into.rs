winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation"
);

#[test]
fn try_into() -> winrt::Result<()> {
    use winrt::TryInto;
    use winrt::ComInterface;

    use windows::foundation::Uri;
    use windows::foundation::IStringable;
    use windows::foundation::IClosable;

    let uri = Uri::create_uri("http://kennykerr.ca")?;

    // Uri implements IStringable so this query should succeed.
    let s: IStringable = uri.try_into().unwrap();
    assert!(s.to_string()? == "http://kennykerr.ca/");

    // Uri does not implement IClosable so this should fail.
    let c: winrt::Result<IClosable> = uri.try_into();
    assert!(c.is_err());

    // And we should be able to query an interface for a class and it should use
    // its default interface GUID to resolve the query.
    let uri2: Uri = s.try_into().unwrap();
    assert!(uri2.domain()? == "kennykerr.ca");

    // Given a null Uri...
    let null_uri = Uri::default();
    assert!(null_uri.is_null());

    // ...the try_into succeeds, but returns a null IStringable.
    let null_s: IStringable = null_uri.try_into().unwrap();
    assert!(null_s.is_null());

    Ok(())
}
