#![allow(dead_code)]

use crate::*;

// TODO: build the libs directly (from def files) rather than relying on the Windows SDK.
#[link(name = "windowsapp")]
extern "system" {
    pub fn CoIncrementMTAUsage(cookie: *mut *mut std::ffi::c_void) -> ErrorCode;
    pub fn RoGetActivationFactory(
        hstring: *const std::ffi::c_void,
        interface: &Guid,
        result: *mut *mut std::ffi::c_void,
    ) -> ErrorCode;

    pub fn WindowsCreateString(
        value: *const u16,
        length: u32,
        result: *mut *mut std::ffi::c_void,
    ) -> ErrorCode;
    pub fn WindowsGetStringRawBuffer(
        hstring: *const std::ffi::c_void,
        length: &mut u32,
    ) -> *const u16;
    pub fn WindowsGetStringLen(hstring: *const std::ffi::c_void) -> u32;
    pub fn WindowsPreallocateStringBuffer(
        len: u32,
        buffer: *mut *mut u16,
        handle: *mut *mut std::ffi::c_void,
    ) -> ErrorCode;
    pub fn WindowsPromoteStringBuffer(
        handle: *const std::ffi::c_void,
        hstring: *mut *mut std::ffi::c_void,
    ) -> ErrorCode;
    pub fn WindowsDeleteString(hstring: *const std::ffi::c_void) -> ErrorCode;
}
