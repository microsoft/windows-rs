#[test]
#[cfg(target_arch = "x86_64")]
fn test() {
    use windows::Win32::System::Diagnostics::Debug::KNONVOLATILE_CONTEXT_POINTERS;
    assert_eq!(256, core::mem::size_of::<KNONVOLATILE_CONTEXT_POINTERS>());

    use windows::Wdk::Foundation::ACCESS_STATE;
    assert_eq!(160, core::mem::size_of::<ACCESS_STATE>());
    assert_eq!(8, core::mem::align_of::<ACCESS_STATE>());
}

#[test]
#[cfg(target_arch = "x86")]
fn test() {
    use windows::Win32::System::Diagnostics::Debug::KNONVOLATILE_CONTEXT_POINTERS;
    assert_eq!(4, core::mem::size_of::<KNONVOLATILE_CONTEXT_POINTERS>());

    use windows::Wdk::Foundation::ACCESS_STATE;
    assert_eq!(116, core::mem::size_of::<ACCESS_STATE>());
    assert_eq!(4, core::mem::align_of::<ACCESS_STATE>());
}
