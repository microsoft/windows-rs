use crate::*;

use bindings::windows::win32::system_services::{
    GetProcessHeap, HeapAlloc, HeapFree, HeapHandle, HeapReAlloc_dwFlags,
};

pub fn heap_alloc(bytes: usize) -> RawPtr {
    // https://github.com/microsoft/win32metadata/issues/322
    // https://github.com/microsoft/win32metadata/issues/331
    unsafe {
        HeapAlloc(
            HeapHandle(GetProcessHeap().0),
            HeapReAlloc_dwFlags::default(),
            bytes,
        )
    }
}

pub unsafe fn heap_free(ptr: RawPtr) {
    HeapFree(GetProcessHeap(), 0, ptr);
}
