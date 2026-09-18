#![cfg(windows)]
use core::mem::offset_of;

#[test]
#[cfg(target_arch = "x86_64")]
#[expect(clippy::unnecessary_literal_unwrap)] // callback type is intentionally being tested
fn test() {
    use windows::Win32::{CONTEXT, IO_STACK_LOCATION_0_16, IO_STACK_LOCATION_0_17};
    use windows::Win32::{
        PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR, VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
    };

    assert_eq!(1232, size_of::<CONTEXT>());
    assert_eq!(16, align_of::<CONTEXT>());
    assert_eq!(0x10, offset_of!(IO_STACK_LOCATION_0_16, IoControlCode));
    assert_eq!(0x20, size_of::<IO_STACK_LOCATION_0_16>());
    assert_eq!(0x08, offset_of!(IO_STACK_LOCATION_0_17, Length));
    assert_eq!(0x10, size_of::<IO_STACK_LOCATION_0_17>());

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
fn test() {
    use windows::Win32::CONTEXT;
    use windows::Win32::{
        PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR, VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
    };

    assert_eq!(716, size_of::<CONTEXT>());

    extern "C" fn callback(_: PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR) -> i32 {
        32
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(32, unsafe {
        callback.unwrap()(PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR::default())
    });
}
