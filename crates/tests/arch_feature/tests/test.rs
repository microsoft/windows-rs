use windows::{Win32::System::Diagnostics::Debug::CONTEXT, Win32::System::Environment::VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD};

#[test]
#[cfg(target_arch = "x86_64")]
fn test() {
    assert_eq!(1232, core::mem::size_of::<CONTEXT>());

    use windows::Win32::System::Environment::VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64;

    extern "system" fn callback(_: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32 {
        64
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(64, unsafe { callback.unwrap()(core::ptr::null()) });
}

#[test]
#[cfg(target_arch = "x86")]
fn test() {
    assert_eq!(716, core::mem::size_of::<CONTEXT>());

    use windows::Win32::System::Environment::VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32;

    extern "system" fn callback(_: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32 {
        32
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(32, unsafe { callback.unwrap()(core::ptr::null()) });
}
