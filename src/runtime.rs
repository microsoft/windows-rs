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
    pub fn LoadLibraryW(name: *const u16) -> RawPtr;
    pub fn GetProcAddress(library: RawPtr, name: *const u8) -> RawPtr;
    pub fn GetProcessHeap() -> RawPtr;
    pub fn HeapAlloc(heap: RawPtr, flags: u32, bytes: usize) -> RawPtr;
    pub fn HeapFree(heap: RawPtr, flags: u32, ptr: RawPtr) -> i32;

    // TODO: get rid of these (not available on Windows 7)
    pub fn CoIncrementMTAUsage(cookie: *mut RawPtr) -> ErrorCode;
    pub fn RoGetActivationFactory(
        hstring: RawPtr,
        interface: &Guid,
        result: *mut RawPtr,
    ) -> ErrorCode;
}
