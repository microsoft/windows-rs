use windows::{core::*, Win32::System::Com::StructuredStorage::*, Win32::UI::Shell::*};

#[test]
fn path() {
    unsafe {
        let result = PathFindSuffixArrayA(s!("path.txt"), &[s!("doc"), s!("txt"), s!("rs")]);
        assert_eq!(result.to_string().unwrap(), "txt");

        let result = PathFindSuffixArrayW(w!("path.rs"), &[w!("doc"), w!("txt"), w!("rs")]);
        assert_eq!(result.to_string().unwrap(), "rs");
    }
}

#[test]
fn variant() -> Result<()> {
    unsafe {
        let variant = InitPropVariantFromStringVector(Some(&[w!("first"), w!("second")]))?;
        let result = PropVariantToBSTR(&variant)?;
        assert_eq!(result, "first; second");
        Ok(())
    }
}
