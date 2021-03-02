use tests::test_component;
use tests::windows::foundation::{IStringable, PropertyValue, Uri};

#[test]
fn class() -> windows::Result<()> {
    let uri = Uri::create_uri("http://kennykerr.ca")?;

    // All WinRT classes are convertible to windows::Object.
    let object: windows::Object = uri.into();

    assert!(object.type_name()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn interface() -> windows::Result<()> {
    let uri = Uri::create_uri("http://kennykerr.ca")?;
    let stringable: IStringable = uri.into();

    // All WinRT interfaces are convertible to windows::Object.
    let object: windows::Object = stringable.into();

    assert!(object.type_name()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn boxing() -> windows::Result<()> {
    let object = PropertyValue::create_string("hello")?;

    assert!(object.type_name()? == "Windows.Foundation.IReference`1<String>");

    Ok(())
}

#[test]
fn object_param() -> windows::Result<()> {
    let uri = Uri::create_uri("http://kennykerr.ca")?;

    let name = test_component::TestRunner::expect_object(&uri)?;
    assert_eq!(name, "Windows.Foundation.Uri");

    let name = test_component::TestRunner::expect_object(uri)?;
    assert_eq!(name, "Windows.Foundation.Uri");

    let uri = Uri::create_uri("http://kennykerr.ca")?;
    let stringable: IStringable = uri.into();

    let name = test_component::TestRunner::expect_object(&stringable)?;
    assert_eq!(name, "Windows.Foundation.Uri");

    let name = test_component::TestRunner::expect_object(stringable)?;
    assert_eq!(name, "Windows.Foundation.Uri");

    Ok(())
}
