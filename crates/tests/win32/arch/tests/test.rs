use test_win32_arch::Windows::Win32::System::SystemServices::KNONVOLATILE_CONTEXT_POINTERS;

#[test]
#[cfg(target_arch = "x86_64")]
fn test() {
    assert_eq!(256, std::mem::size_of::<KNONVOLATILE_CONTEXT_POINTERS>());
}

#[test]
#[cfg(target_arch = "x86")]
fn test() {
    assert_eq!(4, std::mem::size_of::<KNONVOLATILE_CONTEXT_POINTERS>());
}
