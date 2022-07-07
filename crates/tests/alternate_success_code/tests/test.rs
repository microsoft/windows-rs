use windows::{Win32::Foundation::CO_E_NOTINITIALIZED, Win32::System::Ole::*};

// This test validates that the AlternateSuccessCodes attribute is being honored by preserving the original signature
// and not doing any transformation.
#[test]
fn test() {
    let mut effect = DROPEFFECT::default();
    let hr = unsafe { DoDragDrop(None, None, DROPEFFECT_COPY, &mut effect) };
    assert!(hr == CO_E_NOTINITIALIZED);
}
