use test_winrt::Windows::Foundation::{PropertyValue, Uri};
use windows::Interface;

#[test]
fn into() -> windows::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;

    let object = PropertyValue::CreateInspectable(&uri)?; // reference

    let uri = object.cast::<Uri>()?;
    assert!(uri.Domain()? == "kennykerr.ca");

    let object = PropertyValue::CreateInspectable(uri)?; // value

    let uri = object.cast::<Uri>()?;
    assert!(uri.Domain()? == "kennykerr.ca");

    Ok(())
}
