#![cfg(windows)]
#[test]
#[cfg(target_arch = "x86_64")]
#[expect(clippy::unnecessary_literal_unwrap)] // callback type is intentionally being tested
fn test() {
    use windows_sys::ntenclv::{
        PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR, VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
    };
    use windows_sys::winnt::CONTEXT;

    assert_eq!(1232, size_of::<CONTEXT>());

    extern "system" fn callback(_: PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR) -> i32 {
        64
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(64, unsafe { callback.unwrap()(core::ptr::null_mut()) });
}

#[test]
#[cfg(target_arch = "x86")]
#[expect(clippy::unnecessary_literal_unwrap)] // callback type is intentionally being tested
fn test() {
    use windows_sys::ntenclv::{
        PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR, VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
    };
    use windows_sys::winnt::CONTEXT;

    assert_eq!(716, size_of::<CONTEXT>());

    extern "system" fn callback(_: PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR) -> i32 {
        32
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(32, unsafe { callback.unwrap()(core::ptr::null_mut()) });
}
