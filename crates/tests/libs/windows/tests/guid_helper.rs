#![cfg(windows)]
use windows::{core::*, Foundation::*};

// Simple test for the OS-provided implementation of ELEMENT_TYPE_CMOD_REQD parameters.
#[test]
fn test() -> Result<()> {
    let empty = GuidHelper::Empty()?;
    let a = GuidHelper::CreateNewGuid()?;
    let b = GuidHelper::CreateNewGuid()?;

    assert!(!GuidHelper::Equals(a, b)?);
    assert!(!GuidHelper::Equals(a, empty)?);
    assert!(GuidHelper::Equals(GUID::default(), empty)?);
    assert!(GuidHelper::Equals(a, a)?);

    Ok(())
}
