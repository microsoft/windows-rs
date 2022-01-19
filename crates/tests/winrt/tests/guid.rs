use windows::Foundation::GuidHelper;
use windows::core::GUID;

#[test]
fn guid_helper() -> windows::core::Result<()> {
    let a = GuidHelper::CreateNewGuid()?;
    let b = GuidHelper::CreateNewGuid()?;

    assert!(!GuidHelper::Equals(&a, &b)?);
    assert!(GuidHelper::Equals(&a, &a)?);

    Ok(())
}

#[test]
fn guid_from_string() {
    let a = GUID::from("CFF52E04-CCA6-4614-A17E-754910C84A99");

    let b = GUID::from_values(0xCFF52E04, 0xCCA6, 0x4614, [0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99]);

    assert!(a == b);
}
