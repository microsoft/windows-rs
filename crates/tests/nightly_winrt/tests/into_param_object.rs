use windows::core::Interface;
use windows::Foundation::{PropertyValue, Uri};

#[test]
fn into() -> windows::core::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;

    let object = PropertyValue::CreateInspectable(&uri)?; // reference

    let uri = object.cast::<Uri>()?;
    assert!(uri.Domain()? == "kennykerr.ca");

    let object = PropertyValue::CreateInspectable(uri)?; // value

    let uri = object.cast::<Uri>()?;
    assert!(uri.Domain()? == "kennykerr.ca");

    Ok(())
}
