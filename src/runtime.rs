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
    pub fn CoIncrementMTAUsage(cookie: *mut handle) -> ErrorCode;
    pub fn LoadLibraryW(name: *const u16) -> handle;
    pub fn GetProcAddress(library: handle, name: *const u8) -> handle;
    pub fn GetProcessHeap() -> handle;
    pub fn HeapAlloc(heap: handle, flags: u32, bytes: usize) -> handle;
    pub fn HeapFree(heap: handle, flags: u32, ptr: handle) -> i32;

    // TODO: get rid of these
    pub fn RoGetActivationFactory(hstring: handle, interface: &Guid, result: *mut handle) -> ErrorCode;
}
