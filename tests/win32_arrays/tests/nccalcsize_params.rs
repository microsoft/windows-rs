use test_win32_arrays::windows::win32::windows_and_messaging::NCCALCSIZE_PARAMS;

#[test]
#[cfg(target_arch = "x86_64")]
fn test_arch() {
    assert_eq!(std::mem::size_of::<NCCALCSIZE_PARAMS>(), 56);
}

#[test]
#[cfg(target_arch = "x86")]
fn test_arch() {
    assert_eq!(std::mem::size_of::<NCCALCSIZE_PARAMS>(), 52);
}
