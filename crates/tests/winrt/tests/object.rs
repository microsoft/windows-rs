use windows::core::Interface;
use windows::Foundation::{IStringable, PropertyValue, Uri};

#[test]
fn class() -> windows::core::Result<()> {
    let uri = Uri::CreateUri(&windows::core::HSTRING::from("http://kennykerr.ca"))?;

    // All WinRT classes are convertible to windows::core::IInspectable.
    let object: windows::core::IInspectable = uri.cast()?;

    assert!(object.GetRuntimeClassName()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn interface() -> windows::core::Result<()> {
    let uri = Uri::CreateUri(&windows::core::HSTRING::from("http://kennykerr.ca"))?;
    let stringable: IStringable = uri.cast()?;

    // All WinRT interfaces are convertible to windows::core::IInspectable.
    let object: windows::core::IInspectable = stringable.cast()?;

    assert!(object.GetRuntimeClassName()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn boxing() -> windows::core::Result<()> {
    let object = PropertyValue::CreateString(&windows::core::HSTRING::from("hello"))?;

    assert!(object.GetRuntimeClassName()? == "Windows.Foundation.IReference`1<String>");

    Ok(())
}
