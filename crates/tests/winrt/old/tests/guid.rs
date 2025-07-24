use windows::Foundation::GuidHelper;
use windows::core::HRESULT;

#[test]
fn guid_helper() -> Result<(), HRESULT> {
    let a = GuidHelper::CreateNewGuid()?;
    let b = GuidHelper::CreateNewGuid()?;

    assert!(!GuidHelper::Equals(a, b)?);
    assert!(GuidHelper::Equals(a, a)?);

    Ok(())
}
