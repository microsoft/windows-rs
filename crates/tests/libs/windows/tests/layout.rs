#![cfg(windows)]
use windows::Win32::*;

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
    assert_eq!(64, unsafe {
        callback.unwrap()(PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR::default())
    });
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
    assert_eq!(32, unsafe {
        callback.unwrap()(PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR::default())
    });
}

#[test]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
fn pointer_width() {
    assert_eq!(size_of::<DXGI_ADAPTER_DESC1>(), 312);
    assert_eq!(size_of::<NCCALCSIZE_PARAMS>(), 56);
}

#[test]
#[cfg(target_arch = "x86")]
fn pointer_width() {
    assert_eq!(size_of::<DXGI_ADAPTER_DESC1>(), 296);
    assert_eq!(size_of::<NCCALCSIZE_PARAMS>(), 52);
}

#[test]
fn packed() {
    assert_eq!(size_of::<IPV6_ADDRESS_EX>(), 26);
}
