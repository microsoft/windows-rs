#![cfg(all(windows, any(target_arch = "x86", target_arch = "x86_64")))]
use windows_sys::Win32::*;

#[test]
#[cfg(target_arch = "x86_64")]
fn arch() {
    assert_eq!(256, size_of::<KNONVOLATILE_CONTEXT_POINTERS>());
    assert_eq!(160, size_of::<ACCESS_STATE>());
    assert_eq!(8, align_of::<ACCESS_STATE>());
}

#[test]
#[cfg(target_arch = "x86")]
fn arch() {
    assert_eq!(4, size_of::<KNONVOLATILE_CONTEXT_POINTERS>());
    assert_eq!(116, size_of::<ACCESS_STATE>());
    assert_eq!(4, align_of::<ACCESS_STATE>());
}

#[test]
#[cfg(target_arch = "x86_64")]
#[expect(clippy::unnecessary_literal_unwrap)] // callback type is intentionally being tested
fn arch_feature() {
    assert_eq!(1232, size_of::<CONTEXT>());

    extern "C" fn callback(_: PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR) -> i32 {
        64
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(64, unsafe { callback.unwrap()(core::ptr::null_mut()) });
}

#[test]
#[cfg(target_arch = "x86")]
#[expect(clippy::unnecessary_literal_unwrap)] // callback type is intentionally being tested
fn arch_feature() {
    assert_eq!(716, size_of::<CONTEXT>());

    extern "C" fn callback(_: PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR) -> i32 {
        32
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(32, unsafe { callback.unwrap()(core::ptr::null_mut()) });
}
