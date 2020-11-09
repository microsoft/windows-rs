use winrt::foundation::{IStringable, IUriRuntimeClass, Uri};
use winrt::{IAgileObject, Interface, RuntimeName};

#[test]
fn uri() -> winrt::Result<()> {
    assert_eq!(Uri::NAME, "Windows.Foundation.Uri");

    assert_eq!(
        Uri::IID,
        winrt::Guid::from("9E365E57-48B2-4160-956F-C7385120BBFC") // IUriRuntimeClass
    );

    let uri = &Uri::create_uri("http://kennykerr.ca")?;

    assert!(uri.cast::<IAgileObject>().is_ok());
    assert!(uri.domain()? == "kennykerr.ca");
    assert!(uri.port()? == 80);

    // Calls QueryInterface followed by IStringable::ToString under the hood
    assert!(uri.to_string()? == "http://kennykerr.ca/");

    let default: IUriRuntimeClass = uri.into();
    assert!(default.domain()? == uri.domain()?);

    let stringable: IStringable = uri.into();
    assert!(stringable.to_string()? == uri.to_string()?);

    Ok(())
}

#[test]
fn interface_conversion() -> winrt::Result<()> {
    // Convert from Uri class to default interface by value (dropping the uri).
    let uri: Uri = Uri::create_uri("http://kennykerr.ca")?;
    let default: IUriRuntimeClass = uri.into();
    assert!(default.domain()? == "kennykerr.ca");

    // Convert from Uri class to default interface by reference (retaining the uri).
    let uri: &Uri = &Uri::create_uri("http://kennykerr.ca")?;
    let default: IUriRuntimeClass = uri.into();
    assert!(default.domain()? == uri.domain()?);

    // Convert from Uri class to non-default non-generic interface by value.
    let uri: Uri = Uri::create_uri("http://kennykerr.ca")?;
    let default: IStringable = uri.into();
    assert!(default.to_string()? == "http://kennykerr.ca/");

    // Convert from Uri class to non-default non-generic interface by reference.
    let uri: &Uri = &Uri::create_uri("http://kennykerr.ca")?;
    let default: IStringable = uri.into();
    assert!(default.to_string()? == uri.to_string()?);

    // Convert from ??? class to default generic interface by value.

    // Convert from ??? class to default generic interface by reference.

    // Convert from ??? class to non-default generic interface by value.

    // Convert from ??? class to non-default generic interface by reference.

    Ok(())
}
