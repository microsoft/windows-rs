use crate::*;

use bindings::Windows::Win32::System::Memory::{GetProcessHeap, HeapAlloc, HeapFree, HEAP_FLAGS};

pub fn heap_alloc(bytes: usize) -> RawPtr {
    unsafe { HeapAlloc(GetProcessHeap(), HEAP_FLAGS::HEAP_NONE, bytes) }
}

pub unsafe fn heap_free(ptr: RawPtr) {
    HeapFree(GetProcessHeap(), HEAP_FLAGS::HEAP_NONE, ptr);
}
