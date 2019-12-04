#![allow(dead_code)]

use crate::*;

// TODO: build the libs directly (from def files) rather than relying on the Windows SDK.
#[link(name = "windowsapp")]
extern "system" {
    pub fn CoIncrementMTAUsage(cookie: *mut *mut Void) -> ErrorCode;
    pub fn RoGetActivationFactory(hstring: *const Void, interface: &Guid, result: *mut *mut Void) -> ErrorCode;

    pub fn WindowsCreateString(value: *const u16, length: u32, result: *mut *mut Void) -> ErrorCode;
    pub fn WindowsGetStringRawBuffer(hstring: *const Void, length: &mut u32) -> *const u16;
    pub fn WindowsGetStringLen(hstring: *const Void) -> u32;
    pub fn WindowsPreallocateStringBuffer(len: u32, buffer: *mut *mut u16, handle: *mut *mut Void) -> ErrorCode;
    pub fn WindowsPromoteStringBuffer(handle: *const Void, hstring: *mut *mut Void) -> ErrorCode;
    pub fn WindowsDeleteString(hstring: *const Void) -> ErrorCode;
}
