use super::*;
use bindings::Windows::Win32::System::Memory::*;

pub fn heap_alloc(bytes: usize) -> RawPtr {
    unsafe { HeapAlloc(GetProcessHeap(), HEAP_NONE, bytes) }
}

/// # Safety
pub unsafe fn heap_free(ptr: RawPtr) {
    HeapFree(GetProcessHeap(), HEAP_NONE, ptr);
}
