#![allow(dead_code)]

use crate::*;
use std::ffi::c_void;

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
    pub fn LoadLibraryW(name: *const u16) -> *mut c_void;
    pub fn GetProcAddress(library: *mut c_void, name: *const u8) -> *mut c_void;
    pub fn GetProcessHeap() -> *mut c_void;
    pub fn HeapAlloc(heap: *mut c_void, flags: u32, bytes: usize) -> *mut c_void;
    pub fn HeapFree(heap: *mut c_void, flags: u32, ptr: *mut c_void) -> i32;

    // TODO: get rid of these (not available on Windows 7)
    pub fn CoIncrementMTAUsage(cookie: *mut *mut c_void) -> ErrorCode;
    pub fn RoGetActivationFactory(hstring: *mut c_void, interface: *const com::sys::IID, result: *mut *mut c_void) -> ErrorCode;
}
