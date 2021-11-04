use windows::Win32::System::Diagnostics::Debug::CONTEXT;

#[test]
#[cfg(target_arch = "x86_64")]
fn test() {
    assert_eq!(1232, std::mem::size_of::<CONTEXT>());
}

#[test]
#[cfg(target_arch = "x86")]
fn test() {
    assert_eq!(716, std::mem::size_of::<CONTEXT>());
}
