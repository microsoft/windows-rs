#[test]
fn class() -> winrt::Result<()> {
    let uri = winrt::Uri::create_uri("http://kennykerr.ca")?;

    // All WinRT classes are convertible to winrt::Object.
    let object: winrt::Object = uri.into();

    assert!(object.type_name()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn interface() -> winrt::Result<()> {
    let uri = winrt::Uri::create_uri("http://kennykerr.ca")?;
    let stringable: winrt::IStringable = uri.into();

    // All WinRT interfaces are convertible to winrt::Object.
    let object: winrt::Object = stringable.into();

    assert!(object.type_name()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn boxing() -> winrt::Result<()> {
    let object = winrt::PropertyValue::create_string("hello")?;

    assert!(object.type_name()? == "Windows.Foundation.IReference`1<String>");

    Ok(())
}
