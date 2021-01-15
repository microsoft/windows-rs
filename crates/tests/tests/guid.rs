use windows::foundation::GuidHelper;
use windows::Guid;

#[test]
fn guid_helper() -> windows::Result<()> {
    let a = GuidHelper::create_new_guid()?;
    let b = GuidHelper::create_new_guid()?;

    assert!(!GuidHelper::equals(&a, &b)?);
    assert!(GuidHelper::equals(&a, &a)?);

    Ok(())
}

#[test]
fn guid_from_string() {
    let a = Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99");

    let b = Guid::from_values(
        0xCFF52E04,
        0xCCA6,
        0x4614,
        [0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99],
    );

    assert!(a == b);
}
