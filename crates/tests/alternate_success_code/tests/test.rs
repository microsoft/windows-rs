use windows::{Win32::Foundation::CO_E_NOTINITIALIZED, Win32::System::Ole::DoDragDrop};

// This test validates that the AlternateSuccessCodes attribute is being honored by preserving the original signature
// and not doing any transformation.
#[test]
fn test() {
    let mut effect = 0;
    let hr = unsafe { DoDragDrop(None, None, 1, &mut effect) };
    assert!(hr == CO_E_NOTINITIALIZED);
}
