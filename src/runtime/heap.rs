use crate::*;

use bindings::Windows::Win32::SystemServices::{GetProcessHeap, HeapAlloc, HeapFree, HEAP_NONE};

pub fn heap_alloc(bytes: usize) -> RawPtr {
    unsafe { HeapAlloc(GetProcessHeap(), HEAP_NONE, bytes) }
}

pub unsafe fn heap_free(ptr: RawPtr) {
    HeapFree(GetProcessHeap(), HEAP_NONE, ptr);
}
