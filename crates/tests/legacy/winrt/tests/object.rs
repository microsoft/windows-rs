use core::convert::TryInto;
use windows::Foundation::{IStringable, PropertyValue, Uri};

#[test]
fn class() -> windows::core::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;

    // All WinRT classes are convertible to windows::core::IInspectable.
    let object: windows::core::IInspectable = uri.into();

    assert!(object.type_name()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn interface() -> windows::core::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;
    let stringable: IStringable = uri.try_into().unwrap();

    // All WinRT interfaces are convertible to windows::core::IInspectable.
    let object: windows::core::IInspectable = stringable.into();

    assert!(object.type_name()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn boxing() -> windows::core::Result<()> {
    let object = PropertyValue::CreateString("hello")?;

    assert!(object.type_name()? == "Windows.Foundation.IReference`1<String>");

    Ok(())
}
