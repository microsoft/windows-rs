use windows::core::*;
use windows::Foundation::{IClosable, IStringable, Uri};
use windows::Win32::Foundation::*;

#[test]
fn try_into() -> Result<(), HRESULT> {
    let uri = Uri::CreateUri(&windows::core::HSTRING::from("http://kennykerr.ca"))?;

    // Implicit cast to IStringable should work.
    assert!(uri.ToString()? == "http://kennykerr.ca/");

    // Uri implements IStringable so this cast should succeed.
    let s: IStringable = uri.cast()?;
    assert!(s.ToString()? == "http://kennykerr.ca/");

    // Uri does not implement IClosable so this should fail.
    let e = uri.cast::<IClosable>().unwrap_err();
    assert_eq!(e, E_NOINTERFACE);

    // And we should be able to cast an interface for a class and it should use
    // its default interface GUID to resolve the cast.
    let s: IStringable = uri.cast()?;
    let uri: Uri = s.cast()?;
    assert!(uri.Domain()? == "kennykerr.ca");

    Ok(())
}
