use windows::{
    Win32::System::Diagnostics::Debug::CONTEXT,
    Win32::System::Environment::VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
};

#[test]
#[cfg(target_arch = "x86_64")]
fn test() {
    assert_eq!(1232, std::mem::size_of::<CONTEXT>());

    use Win32::System::Environment::VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64;

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = |_: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64| { 64 };
    assert_eq!(64, callback(std::ptr::null()));
}

#[test]
#[cfg(target_arch = "x86")]
fn test() {
    assert_eq!(716, std::mem::size_of::<CONTEXT>());
}
