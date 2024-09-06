use windows::Foundation::GuidHelper;

#[test]
fn guid_helper() -> windows::core::Result<()> {
    let a = GuidHelper::CreateNewGuid()?;
    let b = GuidHelper::CreateNewGuid()?;

    assert!(!GuidHelper::Equals(a, b)?);
    assert!(GuidHelper::Equals(a, a)?);

    Ok(())
}
