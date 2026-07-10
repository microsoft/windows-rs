#![cfg(windows)]
use windows::winuser::NCCALCSIZE_PARAMS;

#[test]
#[cfg(target_arch = "aarch64")]
fn test_arch() {
    assert_eq!(size_of::<NCCALCSIZE_PARAMS>(), 56);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_arch() {
    assert_eq!(size_of::<NCCALCSIZE_PARAMS>(), 56);
}

#[test]
#[cfg(target_arch = "x86")]
fn test_arch() {
    assert_eq!(size_of::<NCCALCSIZE_PARAMS>(), 52);
}
