use windows::core::HRESULT;
use windows::Foundation::GuidHelper;

#[test]
fn guid_helper() -> Result<(), HRESULT> {
    let a = GuidHelper::CreateNewGuid()?;
    let b = GuidHelper::CreateNewGuid()?;

    assert!(!GuidHelper::Equals(a, b)?);
    assert!(GuidHelper::Equals(a, a)?);

    Ok(())
}
