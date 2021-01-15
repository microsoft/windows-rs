use windows::foundation::{PropertyValue, Uri};
use windows::Interface;

#[test]
fn into() -> windows::Result<()> {
    let uri = Uri::create_uri("http://kennykerr.ca")?;

    let object = PropertyValue::create_inspectable(&uri)?; // reference

    let uri = object.cast::<Uri>()?;
    assert!(uri.domain()? == "kennykerr.ca");

    let object = PropertyValue::create_inspectable(uri)?; // value

    let uri = object.cast::<Uri>()?;
    assert!(uri.domain()? == "kennykerr.ca");

    Ok(())
}
