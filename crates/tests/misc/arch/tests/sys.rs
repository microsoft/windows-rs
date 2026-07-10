#![cfg(windows)]
#[test]
#[cfg(target_arch = "x86_64")]
fn test() {
    use windows_sys::Win32::winnt::KNONVOLATILE_CONTEXT_POINTERS;
    assert_eq!(256, size_of::<KNONVOLATILE_CONTEXT_POINTERS>());

    use windows_sys::Wdk::wdm::ACCESS_STATE;
    assert_eq!(160, size_of::<ACCESS_STATE>());
    assert_eq!(8, align_of::<ACCESS_STATE>());
}

#[test]
#[cfg(target_arch = "x86")]
fn test() {
    use windows_sys::Win32::winnt::KNONVOLATILE_CONTEXT_POINTERS;
    assert_eq!(4, size_of::<KNONVOLATILE_CONTEXT_POINTERS>());

    use windows_sys::Wdk::wdm::ACCESS_STATE;
    assert_eq!(116, size_of::<ACCESS_STATE>());
    assert_eq!(4, align_of::<ACCESS_STATE>());
}
