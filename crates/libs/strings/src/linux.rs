#![allow(unused_variables)]

use super::bindings::*;

#[no_mangle]
extern "system" fn GetProcessHeap() -> HANDLE {
    todo!()
}

#[no_mangle]
extern "system" fn HeapAlloc(
    hheap: HANDLE,
    dwflags: HEAP_FLAGS,
    dwbytes: usize,
) -> *mut core::ffi::c_void {
    todo!()
}

#[no_mangle]
extern "system" fn HeapFree(
    hheap: HANDLE,
    dwflags: HEAP_FLAGS,
    lpmem: *const core::ffi::c_void,
) -> BOOL {
    todo!()
}

#[no_mangle]
extern "system" fn SysAllocStringLen(strin: PCWSTR, ui: u32) -> BSTR {
    todo!()
}

#[no_mangle]
extern "system" fn SysFreeString(bstrstring: BSTR) {
    todo!()
}

#[no_mangle]
extern "system" fn SysStringLen(pbstr: BSTR) -> u32 {
    todo!()
}
