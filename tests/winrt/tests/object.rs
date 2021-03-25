use test_winrt::TestComponent;
use test_winrt::Windows::Foundation::{IStringable, PropertyValue, Uri};

#[test]
fn class() -> windows::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;

    // All WinRT classes are convertible to windows::Object.
    let object: windows::Object = uri.into();

    assert!(object.type_name()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn interface() -> windows::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;
    let stringable: IStringable = uri.into();

    // All WinRT interfaces are convertible to windows::Object.
    let object: windows::Object = stringable.into();

    assert!(object.type_name()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn boxing() -> windows::Result<()> {
    let object = PropertyValue::CreateString("hello")?;

    assert!(object.type_name()? == "Windows.Foundation.IReference`1<String>");

    Ok(())
}

#[test]
fn object_param() -> windows::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;

    let name = TestComponent::TestRunner::ExpectObject(&uri)?;
    assert_eq!(name, "Windows.Foundation.Uri");

    let name = TestComponent::TestRunner::ExpectObject(uri)?;
    assert_eq!(name, "Windows.Foundation.Uri");

    let uri = Uri::CreateUri("http://kennykerr.ca")?;
    let stringable: IStringable = uri.into();

    let name = TestComponent::TestRunner::ExpectObject(&stringable)?;
    assert_eq!(name, "Windows.Foundation.Uri");

    let name = TestComponent::TestRunner::ExpectObject(stringable)?;
    assert_eq!(name, "Windows.Foundation.Uri");

    Ok(())
}
