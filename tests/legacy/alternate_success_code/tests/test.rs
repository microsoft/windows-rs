use test_alternate_success_code::*;
use Windows::Win32::Graphics::DirectDraw::CO_E_NOTINITIALIZED;
use Windows::Win32::System::Com::DoDragDrop;

// This test validates that the AlternateSuccessCodes attribute is being honored by preserving the original signature
// and not doing any transformation.
#[test]
fn test() {
    let mut effect = 0;
    let hr = unsafe { DoDragDrop(None, None, 1, &mut effect) };
    assert!(hr == windows::HRESULT(CO_E_NOTINITIALIZED as _));
}
