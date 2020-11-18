use crate::*;

pub fn heap_alloc(bytes: usize) -> RawPtr {
    unsafe { HeapAlloc(GetProcessHeap(), 0, bytes) }
}

pub unsafe fn heap_free(ptr: RawPtr) {
    HeapFree(GetProcessHeap(), 0, ptr);
}

#[link(name = "kernel32")]
extern "system" {
    fn GetProcessHeap() -> RawPtr;
    fn HeapAlloc(heap: RawPtr, flags: u32, bytes: usize) -> RawPtr;
    fn HeapFree(heap: RawPtr, flags: u32, ptr: RawPtr) -> i32;
}
