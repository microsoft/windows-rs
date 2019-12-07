#![allow(dead_code)]

use crate::*;


// TODO: delay load everything via LoadLibrary(combase.dll)/GetProcAddress(function)
// except for LoadLibrary and GetProcAddress from kernel32.dll. For those if they don't 
// link automatically then create a def file and generate a lib rather than relying on the SDK.
//
// From a performance perspective, all APIs needed for WinRT can be delay loaded
// with exception of HSTRING APIs. These cannot be delay loaded but neither can they
// be depended on for portability down to Windows 7. For that, inline an implementation
// of the HSTRING APIs that's compatible with Windows 10 but can also run down level.
//

#[link(name = "windowsapp")]
extern "system" {
    pub fn CoIncrementMTAUsage(cookie: *mut *mut std::ffi::c_void) -> ErrorCode;

    pub fn RoGetActivationFactory(hstring: *const std::ffi::c_void, interface: &Guid, result: *mut *mut std::ffi::c_void) -> ErrorCode;

    pub fn WindowsCreateString(value: *const u16, length: u32, result: *mut *mut std::ffi::c_void) -> ErrorCode;

    pub fn WindowsGetStringRawBuffer(hstring: *const std::ffi::c_void, length: &mut u32) -> *const u16;

    pub fn WindowsGetStringLen(hstring: *const std::ffi::c_void) -> u32;

    pub fn WindowsPreallocateStringBuffer(len: u32, buffer: *mut *mut u16, handle: *mut *mut std::ffi::c_void) -> ErrorCode;

    pub fn WindowsPromoteStringBuffer(handle: *const std::ffi::c_void, hstring: *mut *mut std::ffi::c_void) -> ErrorCode;

    pub fn WindowsDeleteString(hstring: *const std::ffi::c_void) -> ErrorCode;
}
