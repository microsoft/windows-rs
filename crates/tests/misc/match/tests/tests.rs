use windows::Win32::Foundation::*;
use windows_future::*;

#[test]
fn test() {
    assert!(matches!(E_FAIL, E_FAIL));
    assert!(matches!(AsyncStatus::Canceled, AsyncStatus::Canceled));
    assert!(matches!(
        EXCEPTION_ACCESS_VIOLATION,
        EXCEPTION_ACCESS_VIOLATION
    ));
}
