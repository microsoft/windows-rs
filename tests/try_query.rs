use winrt::foundation::{IClosable, IStringable, Uri};
use winrt::ComInterface;

#[test]
fn try_into() -> winrt::Result<()> {
    let uri = Uri::create_uri("http://kennykerr.ca")?;

    // Uri implements IStringable so this query should succeed.
    let s: IStringable = uri.try_query()?;
    assert!(s.to_string()? == "http://kennykerr.ca/");

    // Uri does not implement IClosable so this should fail.
    let c: winrt::Result<IClosable> = uri.try_query();
    assert!(c.is_err());

    // And we should be able to query an interface for a class and it should use
    // its default interface GUID to resolve the query.
    let uri: Uri = s.try_query()?;
    assert!(uri.domain()? == "kennykerr.ca");

    // Given a null Uri...
    let uri = Uri::default();
    assert!(uri.is_null());

    // ...the try_query fails.
    let s: winrt::Result<IStringable> = uri.try_query();
    assert!(s.is_err());

    Ok(())
}
