use test_winrt::Windows::Foundation::{IStringable, IUriRuntimeClass, Uri};
use windows::{IAgileObject, Interface, RuntimeName};
use std::convert::TryInto;

#[test]
fn uri() -> windows::Result<()> {
    assert_eq!(Uri::NAME, "Windows.Foundation.Uri");

    assert_eq!(
        Uri::IID,
        windows::Guid::from("9E365E57-48B2-4160-956F-C7385120BBFC") // IUriRuntimeClass
    );

    let uri = &Uri::CreateUri("http://kennykerr.ca")?;

    assert!(uri.cast::<IAgileObject>().is_ok());
    assert!(uri.Domain()? == "kennykerr.ca");
    assert!(uri.Port()? == 80);

    // Calls QueryInterface followed by IStringable::ToString under the hood
    assert!(uri.ToString()? == "http://kennykerr.ca/");

    let stringable: IStringable = uri.try_into().unwrap();
    assert!(stringable.ToString()? == uri.ToString()?);

    Ok(())
}

#[test]
fn interface_conversion() -> windows::Result<()> {
    // TODO: Find an example where the default constructor is not exclusive.

    // TODO: Convert from ??? class to (non-exclusive) default interface by value (dropping the class).
    let uri: Uri = Uri::CreateUri("http://kennykerr.ca")?;
    let _default: IUriRuntimeClass = uri.cast()?;

    // TODO: Convert from ??? class to (non-exclusive) default interface by reference (retaining the class).
    let uri: &Uri = &Uri::CreateUri("http://kennykerr.ca")?;
    let _default: IUriRuntimeClass = uri.cast()?;

    // Convert from Uri class to non-default non-generic interface by value.
    let uri: Uri = Uri::CreateUri("http://kennykerr.ca")?;
    let default: IStringable = uri.try_into().unwrap();
    assert!(default.ToString()? == "http://kennykerr.ca/");

    // Convert from Uri class to non-default non-generic interface by reference.
    let uri: &Uri = &Uri::CreateUri("http://kennykerr.ca")?;
    let default: IStringable = uri.try_into().unwrap();
    assert!(default.ToString()? == uri.ToString()?);

    // Convert from ??? class to default generic interface by value.

    // Convert from ??? class to default generic interface by reference.

    // Convert from ??? class to non-default generic interface by value.

    // Convert from ??? class to non-default generic interface by reference.

    Ok(())
}
