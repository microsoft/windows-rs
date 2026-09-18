#![cfg(windows)]

#[test]
fn clfs_aligned_layout() {
    use windows_sys::Win32::CLS_SCAN_CONTEXT;

    assert_eq!(8, align_of::<CLS_SCAN_CONTEXT>());
    assert_eq!(56, size_of::<CLS_SCAN_CONTEXT>());
    assert_eq!(0, core::mem::offset_of!(CLS_SCAN_CONTEXT, cidNode));
    assert_eq!(8, core::mem::offset_of!(CLS_SCAN_CONTEXT, hLog));
    assert_eq!(16, core::mem::offset_of!(CLS_SCAN_CONTEXT, cIndex));
    assert_eq!(24, core::mem::offset_of!(CLS_SCAN_CONTEXT, cContainers));
    assert_eq!(
        32,
        core::mem::offset_of!(CLS_SCAN_CONTEXT, cContainersReturned)
    );
    assert_eq!(40, core::mem::offset_of!(CLS_SCAN_CONTEXT, eScanMode));
    assert_eq!(48, core::mem::offset_of!(CLS_SCAN_CONTEXT, pinfoContainer));
}

#[test]
#[cfg(target_arch = "x86_64")]
#[expect(clippy::unnecessary_literal_unwrap)] // callback type is intentionally being tested
fn test() {
    use windows_sys::Win32::CONTEXT;
    use windows_sys::Win32::{
        PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR, VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
    };

    assert_eq!(1232, size_of::<CONTEXT>());
    assert_eq!(16, align_of::<CONTEXT>());

    extern "C" fn callback(_: PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR) -> i32 {
        64
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(64, unsafe { callback.unwrap()(core::ptr::null_mut()) });
}

#[test]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
fn aligned_layout() {
    use windows_sys::Win32::{IO_STACK_LOCATION_0_16, IO_STACK_LOCATION_0_17};

    assert_eq!(8, align_of::<IO_STACK_LOCATION_0_16>());
    assert_eq!(0x20, size_of::<IO_STACK_LOCATION_0_16>());
    assert_eq!(
        0x10,
        core::mem::offset_of!(IO_STACK_LOCATION_0_16, IoControlCode)
    );
    assert_eq!(8, align_of::<IO_STACK_LOCATION_0_17>());
    assert_eq!(0x10, size_of::<IO_STACK_LOCATION_0_17>());
    assert_eq!(0x08, core::mem::offset_of!(IO_STACK_LOCATION_0_17, Length));
}

#[test]
#[cfg(target_arch = "x86")]
#[expect(clippy::unnecessary_literal_unwrap)] // callback type is intentionally being tested
fn test() {
    use windows_sys::Win32::{CONTEXT, IO_STACK_LOCATION_0_16, IO_STACK_LOCATION_0_17};
    use windows_sys::Win32::{
        PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR, VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
    };

    assert_eq!(716, size_of::<CONTEXT>());
    assert_eq!(4, align_of::<IO_STACK_LOCATION_0_16>());
    assert_eq!(0x10, size_of::<IO_STACK_LOCATION_0_16>());
    assert_eq!(
        0x08,
        core::mem::offset_of!(IO_STACK_LOCATION_0_16, IoControlCode)
    );
    assert_eq!(4, align_of::<IO_STACK_LOCATION_0_17>());
    assert_eq!(0x08, size_of::<IO_STACK_LOCATION_0_17>());
    assert_eq!(0x04, core::mem::offset_of!(IO_STACK_LOCATION_0_17, Length));

    extern "C" fn callback(_: PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR) -> i32 {
        32
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(32, unsafe { callback.unwrap()(core::ptr::null_mut()) });
}
