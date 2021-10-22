use test_winrt::Windows::Foundation::{IClosable, IStringable, Uri};
use windows::runtime::{Interface, Result};

#[test]
fn try_into() -> Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;

    // Implicit cast to IStringable should work.
    assert!(uri.ToString()? == "http://kennykerr.ca/");

    // Uri implements IStringable so this cast should succeed.
    let s: Result<IStringable> = uri.cast();
    assert!(s.unwrap().ToString()? == "http://kennykerr.ca/");

    // Uri does not implement IClosable so this should fail.
    let c: Result<IClosable> = uri.cast();
    assert!(c.is_err());

    // And we should be able to cast an interface for a class and it should use
    // its default interface GUID to resolve the cast.
    let s: IStringable = uri.cast()?;
    let uri: Uri = s.cast()?;
    assert!(uri.Domain()? == "kennykerr.ca");

    Ok(())
}
