use windows::Win32::UI::WindowsAndMessaging::NCCALCSIZE_PARAMS;

#[test]
#[cfg(target_arch = "aarch64")]
fn test_arch() {
    assert_eq!(core::mem::size_of::<NCCALCSIZE_PARAMS>(), 56);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_arch() {
    assert_eq!(core::mem::size_of::<NCCALCSIZE_PARAMS>(), 56);
}

#[test]
#[cfg(target_arch = "x86")]
fn test_arch() {
    assert_eq!(core::mem::size_of::<NCCALCSIZE_PARAMS>(), 52);
}
