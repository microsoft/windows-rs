use crate::{hstring, ErrorCode, Guid, RawPtr};

#[link(name = "kernel32")]
extern "system" {
    pub fn GetProcessHeap() -> RawPtr;
    pub fn HeapAlloc(heap: RawPtr, flags: u32, bytes: usize) -> RawPtr;
    pub fn HeapFree(heap: RawPtr, flags: u32, ptr: RawPtr) -> i32;

    pub fn CreateEventW(security: RawPtr, manual: i32, state: i32, name: RawPtr) -> RawPtr;
    pub fn SetEvent(handle: RawPtr) -> i32;
    pub fn WaitForSingleObject(handle: RawPtr, milliseconds: u32) -> u32;
    pub fn CloseHandle(handle: RawPtr) -> i32;

    pub fn LoadLibraryW(name: *const u16) -> RawPtr;
    pub fn FreeLibrary(library: RawPtr) -> i32;
    pub fn GetProcAddress(library: RawPtr, name: *const u8) -> RawPtr;

    pub fn FormatMessageW(
        flags: u32,
        source: RawPtr,
        code: ErrorCode,
        language: u32,
        buffer: *mut *mut u16,
        size: u32,
        args: RawPtr,
    ) -> u32;
}

#[link(name = "windowsapp")]
extern "system" {
    pub fn CoIncrementMTAUsage(cookie: *mut RawPtr) -> ErrorCode;

    pub fn RoGetActivationFactory(
        hstring: *mut hstring::Header,
        interface: &Guid,
        result: *mut RawPtr,
    ) -> ErrorCode;

    pub fn SetRestrictedErrorInfo(info: RawPtr) -> ErrorCode;
    pub fn RoOriginateError(code: ErrorCode, message: RawPtr) -> i32;
}

#[link(name = "oleaut32")]
extern "system" {
    pub fn SysStringLen(bstr: *mut u16) -> u32;
    pub fn SysFreeString(bstr: *mut u16);
    pub fn GetErrorInfo(reserved: u32, info: *mut *mut u16) -> ErrorCode;
}

#[link(name = "ole32")]
extern "system" {
    pub fn CoTaskMemFree(ptr: RawPtr);
}
